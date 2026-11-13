---
sovereignty_schema: "https://github.com"
artifact_id: "memnet-specimen-readme-v1"
maintainer: "Borealis S. Hedling"
upstream_origin: "8b-is/memnet"
methodology: "NDH-Meta-Systems (Five-Boundary Structural Audit & Sovereignty Architecture)"
canonical_doi: "10.5281/zenodo.24149283"
membrane:
  state: "RESEARCH_SPECIMEN"
  activation_allowed: false
  requires_explicit_consent: true
cryptographic_anchor:
  hash_algorithm: "SHA-256"
  signature_scheme: "Ed25519"
  public_key_fingerprint: "SHA256:QUOn/esk9dfLFyc/AwFHMIlK5zbI5JOpZTI5vh3y7Dg"
---

# MEMNET-SPECIMEN — Deterministic Semantic & Contextual Mesh Architecture

**Research Fork & Architectural Refactoring of [`8b-is/memnet`](https://github.com)**  
**Methodology:** `NDH-Meta-Systems` (Five-Boundary Separation of Concerns)  
**Specification Reference:** [`REMEDIATION_ARCHITECTURE.md`](./REMEDIATION_ARCHITECTURE.md)  
**Canonical Anchor:** *Epistemic Coevolution, Statutory Friction, and Sovereign Edge Governance* (`DOI: 10.5281/zenodo.24149283`)

---

## 1. Research Vision & Refactoring Scope
Conventional networks route packets strictly by static topology (`IP` / `MAC`), treating semantic context as an external application concern. The upstream `MEMNET` research hypothesis explores how distributed agents, edge devices, and local knowledge stores can coordinate through **semantic salience, content relevance, and context-aware transport** without collapsing into centralized ontology control.

This specimen fork (`Borealiscodes/memnet-specimen`) preserves that core research inquiry—alongside the upstream repository's zero-dependency **MTU packet batcher** (`PacketBatcher`), **9-state complex ternary reference oracle** (`src/complex_ternary.rs`), and **BM25F + RRF local retrieval engine**—while refactoring the mesh architecture so it operates deterministically, securely, and safely across real edge hardware:

1. **Deterministic \(O(\log N)\) Transport vs. Application-Layer Search:** Separates packet forwarding (XOR DHT / monotonic coordinate routing) from probabilistic lexical ranking (`BM25F + RRF`), preventing routing loops and IDF-induced split-brain drift.
2. **Asymmetric `Ed25519` Identity & Capability Leases:** Replaces shared-secret `HMAC-SHA256` (`SoulSig`) authentication with local `Ed25519` key pairs and time-bounded capability tokens (`Dormant<T>` $\to$ `Consented<T>`), eliminating symmetric secret-sharing spoofing.
3. **Consent-Gated Dormancy & Pull-Based Sync:** Replaces unsolicited push-based `432 Hz` beacon floods (`compassion_sweep`) with first-class `Dormant` power states and pull-based wake synchronization for constrained `LoRa` and `BLE` links.
4. **Zero-Leak Overlay Addressing:** Derives node identifiers strictly from `SHA-256(Ed25519_PubKey)` digests rather than physical hardware serial numbers or private RFC1918 LAN addresses.

---

## 2. Layered Mesh Architecture

| Architectural Layer | Mechanism | Responsibility & Invariant |
| :--- | :--- | :--- |
| **Layer 1: Wire Framing & Ternary Kernel** | `PacketBatcher` + `src/complex_ternary.rs` | Coalesces `.m8` capsules into MTU-aware frames (`LoRa220`, `Ble512`, `Ipv6Tunnel1280`, `SmartTreeSafe1450`) and verifies 9-state bit-plane arithmetic (`{-1,0,+1} + i{-1,0,+1}`). |
| **Layer 2: Deterministic Forwarding Plane** | Monotonic XOR DHT / Spectral Coordinates | Guarantees loop-free convergence and bounded \(O(\log N)\) hop routing without inspecting payload semantics in transit. |
| **Layer 3: Cryptographic Identity & Leases** | `Ed25519` + `allowed_signers` + Scoped Capabilities | Authenticates node identities (`SHA-256(PubKey)`) and resource leases (`ComputeSanctuaryLease`) via verifiable public-key signatures and expiration bounds. |
| **Layer 4: Consent & Power Membrane** | `Dormant<T>` Type-State + Pull Sync | Protects idle or battery-constrained peers from unsolicited broadcast storms; nodes poll for updates strictly upon explicit wake/consent. |
| **Layer 5: Local Semantic Retrieval** | `SafeMemory::retrieve_resonant_ranked` (`BM25F + RRF`) | Operates exclusively as a **local application-layer index** over content-addressed (`SHA-256`) capsules—never as a network routing oracle. |

---

## 3. Preserved Upstream Components & Research Lineage
* **Upstream Attribution:** Originated in [`8b-is/memnet`](https://github.com) (incorporating conceptual monographs by *Flyxion / standardgalactic* and multi-language Rust/Node/Python protocol engines).
* **Retained Verification Primitives:**
  * `src/complex_ternary.rs` — 17 deterministic property tests covering bit-plane validity, Q16.16 saturation, NAND gate cascades, and epoch-chaining seals.
  * `PacketBatcher` — Zero-fragmentation MTU frame packing (`95.45%` wire efficiency across 1,000 capsules).
  * `SafeMemory` — Field-weighted `BM25F` + ordered-bigram proximity + Reciprocal Rank Fusion (`RRF`, $k=60$) local document ranking.

---

## 4. Building & Running Offline Verification Tests

### Rust Core & Complex Ternary Oracle
```bash
cargo test
```

### JavaScript / Node.js SDK
```bash
node --test sdk/js/memnet.test.mjs
```

### Python SDK
```bash
python3 -m unittest discover -s sdk/python -v
```

---

## 5. Governance & Non-Activation Boundary
* **Research Specimen Posture:** This repository is maintained as a forensic and architectural research fork under the **Author Sovereignty & Collaboration Ecosystem Charter (v1.1)**.
* **Non-Activation Clause:** Descriptive semantic schemas (`NDH-Meta-Systems`) remain strictly separated from active packet routing (`Sealed-Layer Neutral`), ensuring semantic salience metrics never centralize epistemic authority or silently filter network reachability.

*Maintainer: Borealis S. Hedling · Dublin, Ireland · November 2026*
