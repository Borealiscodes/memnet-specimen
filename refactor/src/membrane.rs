//! Compile-Time Dormancy & Pull-Based Consent Membrane
//! Replaces unsolicited push-beacon floods (`compassion_sweep`) with type-state
//! `Dormant -> Consented` transitions and exponential backoff polling.
//! Methodology: NDH-Meta-Systems (Sealed-Layer Neutral)

#![no_std]

use core::marker::PhantomData;
use crate::identity::{CapabilityGrant, CapabilityScope, IdentityError, NodeId};

/// Type-level state markers (zero runtime memory overhead).
pub struct Dormant;
pub struct Consented;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MembraneError {
    Identity(IdentityError),
    BackoffActive { next_allowed_unix: u64 },
}

impl From for MembraneError {
    fn from(err: IdentityError) -> Self {
        Self::Identity(err)
    }
}

/// Exponential backoff schedule preventing broadcast storms on LoRa / BLE links.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PollBackoff {
    pub base_interval_sec: u64,
    pub max_interval_sec: u64,
    pub consecutive_empty_polls: u32,
    pub last_poll_unix: u64,
}

impl PollBackoff {
    pub const fn new(base_interval_sec: u64, max_interval_sec: u64) -> Self {
        Self {
            base_interval_sec,
            max_interval_sec,
            consecutive_empty_polls: 0,
            last_poll_unix: 0,
        }
    }

    /// Computes next permitted poll timestamp using integer bit-shift doubling (zero floats).
    pub const fn next_allowed_unix(&self) -> u64 {
        if self.last_poll_unix == 0 {
            return 0;
        }
        let shift = if self.consecutive_empty_polls > 16 {
            16
        } else {
            self.consecutive_empty_polls
        };
        let step = self.base_interval_sec.saturating_mul(1u64 << shift);
        let clamped = if step > self.max_interval_sec {
            self.max_interval_sec
        } else {
            step
        };
        self.last_poll_unix.saturating_add(clamped)
    }

    pub fn record_poll(&mut self, now_unix: u64, received_updates: bool) {
        self.last_poll_unix = now_unix;
        if received_updates {
            self.consecutive_empty_polls = 0;
        } else {
            self.consecutive_empty_polls = self.consecutive_empty_polls.saturating_add(1);
        }
    }
}

/// Consent-gated peer session.
/// While in `Dormant` state, outbound network polling methods do not exist at compile time.
pub struct PeerMembrane<State = Dormant> {
    pub local_node: NodeId,
    pub remote_peer: NodeId,
    pub backoff: PollBackoff,
    _state: PhantomData<State>,
}

impl PeerMembrane<Dormant> {
    /// Initializes every peer link in unconditional `Dormant` state.
    pub const fn new_dormant(
        local_node: NodeId,
        remote_peer: NodeId,
        base_interval_sec: u64,
        max_interval_sec: u64,
    ) -> Self {
        Self {
            local_node,
            remote_peer,
            backoff: PollBackoff::new(base_interval_sec, max_interval_sec),
            _state: PhantomData,
        }
    }

    /// Transitions `Dormant -> Consented` only when a valid Ed25519 `SYNC_PULL` grant
    /// is verified against the remote peer's public key.
    pub fn wake_with_consent(
        self,
        remote_pubkey_bytes: &[u8; 32],
        grant: &CapabilityGrant,
        now_unix: u64,
    ) -> Result<PeerMembrane<Consented>, MembraneError> {
        grant.verify(
            remote_pubkey_bytes,
            &self.local_node,
            CapabilityScope::SYNC_PULL,
            now_unix,
        )?;

        Ok(PeerMembrane {
            local_node: self.local_node,
            remote_peer: self.remote_peer,
            backoff: self.backoff,
            _state: PhantomData,
        })
    }
}

impl PeerMembrane<Consented> {
    /// Pull-based wake sync: initiated exclusively by the local edge node,
    /// respecting exponential backoff to protect battery and radio airtime.
    pub fn request_pull_sync(&mut self, now_unix: u64) -> Result<NodeId, MembraneError> {
        let next_allowed = self.backoff.next_allowed_unix();
        if now_unix < next_allowed {
            return Err(MembraneError::BackoffActive {
                next_allowed_unix: next_allowed,
            });
        }
        Ok(self.remote_peer)
    }

    /// Updates backoff timer after a pull attempt completes.
    pub fn complete_pull(&mut self, now_unix: u64, received_updates: bool) {
        self.backoff.record_poll(now_unix, received_updates);
    }

    /// Unconditional, zero-cost return to `Dormant` sleep state at any time.
    pub fn return_to_dormant(self) -> PeerMembrane<Dormant> {
        PeerMembrane {
            local_node: self.local_node,
            remote_peer: self.remote_peer,
            backoff: self.backoff,
            _state: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    #[test]
    fn dormant_gate_enforces_consent_and_backoff() {
        let peer_sk = SigningKey::from_bytes(&[3u8; 32]);
        let peer_vk = peer_sk.verifying_key();
        let peer_id = NodeId::from_verifying_key(&peer_vk);

        let local_vk = SigningKey::from_bytes(&[5u8; 32]).verifying_key();
        let local_id = NodeId::from_verifying_key(&local_vk);

        let dormant = PeerMembrane::new_dormant(local_id, peer_id, 10, 300);

        let mut grant = CapabilityGrant {
            grantor: peer_id,
            grantee: local_id,
            scope: CapabilityScope(CapabilityScope::SYNC_PULL),
            max_memory_mb: 64,
            expires_at_unix: 2_000_000_000,
            signature_bytes: [0u8; 64],
        };
        grant.signature_bytes = peer_sk.sign(&grant.canonical_payload()).to_bytes();

        let mut active = dormant
            .wake_with_consent(peer_vk.as_bytes(), &grant, 1_000)
            .expect("valid consent token wakes membrane");

        // First pull at t=1000 succeeds; empty result sets next backoff to 1000 + 10 = 1010
        assert_eq!(active.request_pull_sync(1_000), Ok(peer_id));
        active.complete_pull(1_000, false);

        // Polling too early at t=1005 is blocked
        assert_eq!(
            active.request_pull_sync(1_005),
            Err(MembraneError::BackoffActive {
                next_allowed_unix: 1_010
            })
        );

        // Polling at t=1010 succeeds, and unconditional return to Dormant works cleanly
        assert_eq!(active.request_pull_sync(1_010), Ok(peer_id));
        let _re_sealed: PeerMembrane<Dormant> = active.return_to_dormant();
    }
}
