//! MEMNET-SPECIMEN — Refactored Deterministic Mesh Core (`refactor/src/lib.rs`)
//!
//! Enforces the NDH-Meta-Systems Five-Boundary Separation of Concerns:
//! 1. `identity` — Asymmetric Ed25519 public-key node IDs & scoped capability tokens.
//! 2. `membrane` — Compile-time `Dormant -> Consented` power & pull-sync gates.
//! 3. `routing`  — Monotonic 256-bit XOR Kademlia distance forwarding (`O(log N)`).
//! 4. `LocalContentStore` — Strictly local, content-addressed (`SHA-256`) capsule lookup,
//!    completely isolated from network packet routing.
//!
//! Methodology: NDH-Meta-Systems (Sealed-Layer Neutral)
//! Canonical Reference: DOI 10.5281/zenodo.24149283

#![no_std]

pub mod identity;
pub mod membrane;
pub mod routing;

use sha2::{Digest, Sha256};

pub use identity::{CapabilityGrant, CapabilityScope, IdentityError, NodeId};
pub use membrane::{Consented, Dormant, MembraneError, PeerMembrane, PollBackoff};
pub use routing::{DeterministicRouter, RoutingError, XorDistance};

/// Immutable 256-bit content-addressed envelope identifier (`SHA-256(capsule_bytes)`).
/// Replaces mutable `wave://` semantic URIs in transport headers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentDigest(pub [u8; 32]);

impl ContentDigest {
    pub fn of_bytes(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let digest = hasher.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&digest);
        Self(out)
    }
}

/// Application-layer local index entry.
/// Bound strictly to local node queries—never inspected by `DeterministicRouter`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalIndexEntry {
    pub cid: ContentDigest,
    pub author: NodeId,
    pub created_unix: u64,
    pub relevance_score_bps: u32, // Exact basis points (0..=10000), zero floating-point math
}

/// Fixed-capacity, heap-free local application index (`#![no_std]`).
pub struct LocalContentIndex<const CAPACITY: usize> {
    entries: [Option<LocalIndexEntry>; CAPACITY],
    count: usize,
}

impl<const CAPACITY: usize> LocalContentIndex<CAPACITY> {
    pub const fn new() -> Self {
        Self {
            entries: [None; CAPACITY],
            count: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.count
    }

    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Inserts a verified, content-addressed entry into the local application index.
    pub fn insert(&mut self, entry: LocalIndexEntry) -> bool {
        let mut i = 0;
        while i < CAPACITY {
            if let Some(existing) = self.entries[i] {
                if existing.cid == entry.cid {
                    self.entries[i] = Some(entry);
                    return true;
                }
            }
            i += 1;
        }

        let mut slot = 0;
        while slot < CAPACITY {
            if self.entries[slot].is_none() {
                self.entries[slot] = Some(entry);
                self.count += 1;
                return true;
            }
            slot += 1;
        }
        false
    }

    /// Exact content-addressed lookup (`O(N)` scan over bounded local capacity).
    pub fn get_by_cid(&self, cid: &ContentDigest) -> Option<LocalIndexEntry> {
        let mut i = 0;
        while i < CAPACITY {
            if let Some(entry) = self.entries[i] {
                if &entry.cid == cid {
                    return Some(entry);
                }
            }
            i += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    #[test]
    fn end_to_end_sovereign_mesh_flow() {
        // 1. Derive asymmetric Ed25519 identities (zero hardware/LAN leakage)
        let hub_sk = SigningKey::from_bytes(&[11u8; 32]);
        let hub_vk = hub_sk.verifying_key();
        let hub_id = NodeId::from_verifying_key(&hub_vk);

        let edge_sk = SigningKey::from_bytes(&[22u8; 32]);
        let edge_vk = edge_sk.verifying_key();
        let edge_id = NodeId::from_verifying_key(&edge_vk);

        // 2. Register peer in deterministic XOR router
        let mut router = DeterministicRouter::<8>::new(edge_id);
        router.insert_peer(hub_id).unwrap();
        assert_eq!(router.next_hop(&hub_id), Ok(hub_id));

        // 3. Peer membrane starts Dormant; wakes only with verified Ed25519 grant
        let dormant = PeerMembrane::new_dormant(edge_id, hub_id, 30, 600);
        let mut grant = CapabilityGrant {
            grantor: hub_id,
            grantee: edge_id,
            scope: CapabilityScope(CapabilityScope::SYNC_PULL),
            max_memory_mb: 128,
            expires_at_unix: 2_000_000_000,
            signature_bytes: [0u8; 64],
        };
        grant.signature_bytes = hub_sk.sign(&grant.canonical_payload()).to_bytes();

        let mut active = dormant
            .wake_with_consent(hub_vk.as_bytes(), &grant, 1_500_000_000)
            .expect("valid grant wakes dormant peer");
        assert_eq!(active.request_pull_sync(1_500_000_000), Ok(hub_id));

        // 4. Store content-addressed capsule in local-only index (zero float basis points)
        let cid = ContentDigest::of_bytes(b"ndh-meta-systems-sealed-capsule-v1");
        let mut index = LocalContentIndex::<16>::new();
        assert!(index.insert(LocalIndexEntry {
            cid,
            author: hub_id,
            created_unix: 1_500_000_000,
            relevance_score_bps: 9545, // 95.45% exact integer basis points
        }));

        let retrieved = index.get_by_cid(&cid).expect("cid exists locally");
        assert_eq!(retrieved.relevance_score_bps, 9545);
    }
}
