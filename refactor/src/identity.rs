//! Asymmetric Ed25519 Node Identity & Scoped Capability Verification
//! Replaces symmetric HMAC-SHA256 (`SoulSig`) with non-repudiable public-key cryptography.
//! Methodology: NDH-Meta-Systems (Sealed-Layer Neutral)

#![no_std]

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

/// 256-bit deterministic overlay node identifier derived strictly from `SHA-256(Ed25519_PubKey)`.
/// Eliminates hardware serial number and RFC1918 LAN IP leakage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub [u8; 32]);

impl NodeId {
    /// Derives a collision-resistant `NodeId` from a 32-byte Ed25519 public verifying key.
    pub fn from_verifying_key(key: &VerifyingKey) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let digest = hasher.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&digest);
        Self(out)
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityError {
    MalformedPublicKey,
    MalformedSignature,
    InvalidSignature,
    GrantExpired,
    GrantorMismatch,
    GranteeMismatch,
    ScopeNotPermitted,
}

/// Bitmask flags defining explicit, least-privilege capability scopes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CapabilityScope(pub u8);

impl CapabilityScope {
    pub const SYNC_PULL: u8 = 0b0000_0001;
    pub const COMPUTE_LEASE: u8 = 0b0000_0010;
    pub const ROUTE_RELAY: u8 = 0b0000_0100;

    pub const fn allows(self, flag: u8) -> bool {
        (self.0 & flag) == flag
    }
}

/// Time-bounded, resource-capped Ed25519 capability grant replacing shared-secret leases.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CapabilityGrant {
    pub grantor: NodeId,
    pub grantee: NodeId,
    pub scope: CapabilityScope,
    pub max_memory_mb: u32,
    pub expires_at_unix: u64,
    pub signature_bytes: [u8; 64],
}

impl CapabilityGrant {
    /// Canonical 81-byte deterministic signing payload (zero-float, big-endian).
    /// Layout: [grantor: 32B | grantee: 32B | scope: 1B | max_memory_mb: 4B | expires_at_unix: 8B | domain_tag: 4B]
    pub fn canonical_payload(&self) -> [u8; 81] {
        let mut buf = [0u8; 81];
        buf[0..32].copy_from_slice(&self.grantor.0);
        buf[32..64].copy_from_slice(&self.grantee.0);
        buf[64] = self.scope.0;
        buf[65..69].copy_from_slice(&self.max_memory_mb.to_be_bytes());
        buf[69..77].copy_from_slice(&self.expires_at_unix.to_be_bytes());
        buf[77..81].copy_from_slice(b"NDH1");
        buf
    }

    /// Verifies grantor authenticity, grantee binding, expiration, required scope, and Ed25519 signature.
    pub fn verify(
        &self,
        grantor_pubkey_bytes: &[u8; 32],
        expected_grantee: &NodeId,
        required_scope: u8,
        now_unix: u64,
    ) -> Result {
        let verifying_key = VerifyingKey::from_bytes(grantor_pubkey_bytes)
            .map_err(|_| IdentityError::MalformedPublicKey)?;

        if NodeId::from_verifying_key(&verifying_key) != self.grantor {
            return Err(IdentityError::GrantorMismatch);
        }
        if &self.grantee != expected_grantee {
            return Err(IdentityError::GranteeMismatch);
        }
        if now_unix > self.expires_at_unix {
            return Err(IdentityError::GrantExpired);
        }
        if !self.scope.allows(required_scope) {
            return Err(IdentityError::ScopeNotPermitted);
        }

        let sig = Signature::from_bytes(&self.signature_bytes);
        let payload = self.canonical_payload();
        verifying_key
            .verify(&payload, &sig)
            .map_err(|_| IdentityError::InvalidSignature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    #[test]
    fn ed25519_grant_verifies_and_rejects_tampering() {
        let grantor_sk = SigningKey::from_bytes(&[7u8; 32]);
        let grantor_vk = grantor_sk.verifying_key();
        let grantor_id = NodeId::from_verifying_key(&grantor_vk);

        let grantee_vk = SigningKey::from_bytes(&[9u8; 32]).verifying_key();
        let grantee_id = NodeId::from_verifying_key(&grantee_vk);

        let mut grant = CapabilityGrant {
            grantor: grantor_id,
            grantee: grantee_id,
            scope: CapabilityScope(CapabilityScope::SYNC_PULL | CapabilityScope::COMPUTE_LEASE),
            max_memory_mb: 512,
            expires_at_unix: 2_000_000_000,
            signature_bytes: [0u8; 64],
        };

        let sig = grantor_sk.sign(&grant.canonical_payload());
        grant.signature_bytes = sig.to_bytes();

        // Valid verification succeeds
        assert_eq!(
            grant.verify(
                grantor_vk.as_bytes(),
                &grantee_id,
                CapabilityScope::COMPUTE_LEASE,
                1_900_000_000
            ),
            Ok(())
        );

        // Expired timestamp fails closed
        assert_eq!(
            grant.verify(
                grantor_vk.as_bytes(),
                &grantee_id,
                CapabilityScope::COMPUTE_LEASE,
                2_000_000_001
            ),
            Err(IdentityError::GrantExpired)
        );

        // Tampered memory ceiling fails signature check
        let mut tampered = grant;
        tampered.max_memory_mb = 4096;
        assert_eq!(
            tampered.verify(
                grantor_vk.as_bytes(),
                &grantee_id,
                CapabilityScope::COMPUTE_LEASE,
                1_900_000_000
            ),
            Err(IdentityError::InvalidSignature)
        );
    }
}
