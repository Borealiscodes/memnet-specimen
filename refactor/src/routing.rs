//! Deterministic 256-Bit XOR Distance Router (Kademlia-Style Monotonic Forwarding)
//! Replaces probabilistic lexical search (`BM25F + RRF`) in the forwarding plane
//! with strict monotonic distance convergence (`next_dist  0` for `x != y`
/// 3. `d(x, y) == d(y, x)` (symmetry)
/// 4. `d(x, z) <= d(x, y) ^ d(y, z)` (triangle inequality)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XorDistance(pub [u8; 32]);

impl XorDistance {
    pub const ZERO: Self = Self([0u8; 32]);

    /// Computes bitwise XOR distance between two 256-bit `NodeId`s.
    pub fn between(a: &NodeId, b: &NodeId) -> Self {
        let mut out = [0u8; 32];
        let mut i = 0;
        while i < 32 {
            out[i] = a.0[i] ^ b.0[i];
            i += 1;
        }
        Self(out)
    }

    /// Returns the bucket index (0..256) corresponding to the most significant differing bit,
    /// or `None` if `self == XorDistance::ZERO`.
    pub fn leading_bit_index(&self) -> Option<usize> {
        let mut byte_idx = 0;
        while byte_idx < 32 {
            let b = self.0[byte_idx];
            if b != 0 {
                let leading_zeros = b.leading_zeros() as usize;
                let bit_pos_from_msb = byte_idx * 8 + leading_zeros;
                return Some(255 - bit_pos_from_msb);
            }
            byte_idx += 1;
        }
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoutingError {
    TableFull,
    SelfInsertionIgnored,
    NoCloserNextHop,
}

/// Fixed-capacity, heap-free routing table (`#![no_std]` friendly for embedded / edge nodes).
pub struct DeterministicRouter<const MAX_PEERS: usize> {
    pub local_id: NodeId,
    peers: [Option<NodeId>; MAX_PEERS],
    count: usize,
}

impl<const MAX_PEERS: usize> DeterministicRouter<MAX_PEERS> {
    pub const fn new(local_id: NodeId) -> Self {
        Self {
            local_id,
            peers: [None; MAX_PEERS],
            count: 0,
        }
    }

    pub const fn peer_count(&self) -> usize {
        self.count
    }

    /// Registers a verified `NodeId` in the routing table (idempotent, ignores self).
    pub fn insert_peer(&mut self, peer: NodeId) -> Result<(), RoutingError> {
        if peer == self.local_id {
            return Err(RoutingError::SelfInsertionIgnored);
        }

        let mut i = 0;
        while i < MAX_PEERS {
            if self.peers[i] == Some(peer) {
                return Ok(());
            }
            i += 1;
        }

        let mut slot = 0;
        while slot < MAX_PEERS {
            if self.peers[slot].is_none() {
                self.peers[slot] = Some(peer);
                self.count += 1;
                return Ok(());
            }
            slot += 1;
        }

        Err(RoutingError::TableFull)
    }

    /// Selects the single closest next-hop peer to `target` under the 256-bit XOR metric.
    /// Guarantees strict monotonic progress (`d(next_hop, target) < d(local_id, target)`),
    /// making routing loops mathematically impossible.
    pub fn next_hop(&self, target: &NodeId) -> Result<NodeId, RoutingError> {
        let local_dist = XorDistance::between(&self.local_id, target);
        if local_dist == XorDistance::ZERO {
            return Ok(self.local_id);
        }

        let mut best_peer: Option<NodeId> = None;
        let mut best_dist = local_dist;

        let mut i = 0;
        while i < MAX_PEERS {
            if let Some(candidate) = self.peers[i] {
                let candidate_dist = XorDistance::between(&candidate, target);
                if candidate_dist < best_dist {
                    best_dist = candidate_dist;
                    best_peer = Some(candidate);
                }
            }
            i += 1;
        }

        best_peer.ok_or(RoutingError::NoCloserNextHop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_routing_guarantees_strict_monotonic_progress() {
        let local = NodeId([0x00; 32]);
        let mut target_bytes = [0x00; 32];
        target_bytes[0] = 0b1010_0000;
        let target = NodeId(target_bytes);

        let mut far_bytes = [0x00; 32];
        far_bytes[0] = 0b1111_0000; // XOR with target = 0b0101_0000
        let far_peer = NodeId(far_bytes);

        let mut close_bytes = [0x00; 32];
        close_bytes[0] = 0b1010_0001; // XOR with target = 0b0000_0001
        let close_peer = NodeId(close_bytes);

        let mut router = DeterministicRouter::<8>::new(local);
        router.insert_peer(far_peer).unwrap();
        router.insert_peer(close_peer).unwrap();

        // Selects `close_peer` because XOR distance strictly decreases
        let chosen = router.next_hop(&target).expect("closer hop exists");
        assert_eq!(chosen, close_peer);
        assert!(
            XorDistance::between(&chosen, &target) < XorDistance::between(&local, &target)
        );
    }

    #[test]
    fn xor_routing_refuses_loop_when_no_closer_peer_exists() {
        let local = NodeId([0x00; 32]);
        let mut target_bytes = [0x00; 32];
        target_bytes[31] = 0x01; // distance from local = 1
        let target = NodeId(target_bytes);

        let mut worse_bytes = [0x00; 32];
        worse_bytes[0] = 0x80; // much further away
        let worse_peer = NodeId(worse_bytes);

        let mut router = DeterministicRouter::<4>::new(local);
        router.insert_peer(worse_peer).unwrap();

        // Fails closed instead of bouncing packets backward in a loop
        assert_eq!(router.next_hop(&target), Err(RoutingError::NoCloserNextHop));
    }
}
