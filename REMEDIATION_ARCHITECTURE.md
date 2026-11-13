# Architectural Audit & Remediation Specification: Deterministic Constellation Routing & Long-Term Coherence (v1.0)
**Target Repository:** `Borealiscodes/memnet` (Research Fork of `8b-is/memnet`)  
**Analytical & Engineering Methodology:** `NDH-Meta-Systems` (Five-Boundary Structural Audit & Sovereignty Architecture)  
**Canonical Reference:** *Epistemic Coevolution, Statutory Friction, and Sovereign Edge Governance* (`DOI: 10.5281/zenodo.24149283`)  
**Cryptographic Anchor:** `Ed25519` (`SHA256:QUOn/esk9dfLFyc/AwFHMIlK5zbI5JOpZTI5vh3y7Dg`)  
**Governance Posture:** Constructive • Falsifiable • Altitude-Differentiated (`Author Sovereignty Charter v1.1`)  

---

## 1. Executive Summary & Methodology
This research fork evaluates and refactors the upstream `MEMNET/1.0` prototype using the **NDH-Meta-Systems Five-Boundary Methodology** (Epistemic, Pedagogical, Ethical, Mathematical, and Computational separation of concerns).

While the upstream `memnet` codebase contributes useful low-level primitives—such as zero-dependency MTU packet coalescing (`PacketBatcher`) and bit-packed ternary verification tests (`src/complex_ternary.rs`)—its core routing architecture collapses **packet forwarding**, **probabilistic lexical search (`BM25F + RRF`)**, **affective signaling (`432 Hz` sweeps)**, and **symmetric key authentication (`HMAC-SHA256`)** into a single undifferentiated layer. Under the **NDH-Meta-Systems** methodology, long-term constellation coherence requires strict architectural separation between deterministic transport, content-addressed indexing, asymmetric cryptographic identity, and consent-gated dormancy.

---

## 2. NDH-Meta-Systems Five-Boundary Baseline Audit

| NDH Boundary Layer | Upstream Prototype Behavior (`MEMNET/1.0`) | Identified Structural Instability |
| :--- | :--- | :--- |
| **1. Epistemic** | Resolves packet destinations via lexical search (`BM25F + RRF`) and affective phase coherence (`WaveVad`). | **Non-Deterministic Routing Drift:** Because corpus term frequencies (IDF) shift as capsules enter or expire, identical `wave://` intents resolve to different destinations across peers, producing split-brain state and silent packet drops. |
| **2. Pedagogical** | Interleaves systems engineering (`PacketBatcher`, Rust/Node/Python SDKs) with pop-lore tokens (`0o0-SSU`, `SoulSig`) and hardcoded hardware IDs. | **Cognitive & Operational Opacity:** Obscures mathematical invariants behind thematic lore while leaking physical device serial numbers and private LAN IPs (`192.168.11.x`) in public documentation. |
| **3. Ethical** | `compassion_sweep` broadcasts unsolicited `432 Hz` beacons whenever a peer is silent (`Δt > τ`) or has `neighbor_links `). Retain `SafeMemory::retrieve_resonant_ranked` (`BM25F + Proximity + RRF`) strictly as a **local, application-layer discovery index**—never as a network routing oracle.

### Layer 2: Asymmetric Identity & Scoped Capability Tokens (Computational Hardening)
* **Asymmetric `Ed25519` Node Identities:** Replace shared-secret `HMAC-SHA256` (`SoulSig`) authentication with **asymmetric Ed25519 key pairs** generated locally within each node's hardware sandbox. Peers verify signatures against a public `allowed_signers` registry, eliminating symmetric secret sharing.
* **Cryptographic Capability Leases:** Replace shared-secret `ComputeSanctuaryLease` grants with **Ed25519-signed capability tokens** scoped by exact resource ceiling, expiration timestamp (`expires_at_unix`), and recipient public key (`Dormant<T>` $\to$ `Consented<T>` type-state transition).

### Layer 3: Consent-Gated Dormancy & Backpressure Hygiene (Ethical Alignment)
* **First-Class `Dormant` State:** Replace push-based `compassion_sweep` beacon floods with **consent-gated, pull-based wake synchronization**. Silent or low-power nodes (`LoRa220`, `Ble512`) remain undisturbed until they explicitly initiate an outbound poll.
* **Exponential Backoff & Token-Bucket Rate Limiting:** Enforce strict exponential backoff on neighbor discovery probes so isolated nodes never induce broadcast storms.

### Layer 4: Zero-Leak OpSec & Non-Activating Metadata Membranes (Pedagogical Clarity)
* **Sanitized Documentation & Ephemeral Overlay IDs:** Strip all hardcoded physical hardware serials, personal device names, and private RFC1918 LAN addresses from repository documentation and fixtures. Derive overlay node identifiers exclusively from `SHA-256(Ed25519_PubKey)`.
* **Sealed-Layer Neutrality (`Non-Activation Clause`):** Maintain a strict boundary between descriptive semantic ontologies (`NDH-Meta-Systems`) and active packet forwarding so semantic salience weights cannot silently censor or capture network topology.

---

## 4. Target Refactoring Matrix (`Borealiscodes/memnet`)

| Subsystem | Upstream `MEMNET/1.0` | `NDH-Meta-Systems` Fork Target |
| :--- | :--- | :--- |
| **Routing Plane** | Probabilistic `BM25F + RRF` lexical ranking. | Deterministic monotonic XOR DHT / spectral routing (\(O(\log N)\)). |
| **Search & Retrieval** | Coupled directly into packet routing. | Isolated as a local, opt-in application index (`retrieve_resonant_ranked`). |
| **Authentication** | Symmetric `HMAC-SHA256` (`SoulSig`). | Asymmetric `Ed25519` signatures + `allowed_signers` + scoped capability tokens. |
| **Idle / Edge Nodes** | Unsolicited `432 Hz` beacon floods (`compassion_sweep`). | Consent-gated `Dormant<T>` state + pull-based wake sync. |
| **Node Identifiers** | Hardcoded hardware serials & LAN IPs. | Ephemeral `SHA-256(Ed25519_PubKey)` digests + sanitized test fixtures. |

---

# 🧾 **Provenance Footer — `Borealiscodes/memnet` Remediation Specification (v1.0)**

---
Artifact-Class: Five-Boundary Architectural Audit & Refactoring Specification
Artifact-Name: REMEDIATION_ARCHITECTURE
Target-Repository: Borealiscodes/memnet (Research Fork of 8b-is/memnet)
Methodology: NDH-Meta-Systems (Five-Boundary Structural Audit & Sovereignty Architecture)
Version: v1.0
Altitude: A6–A8 (Forensic • Distributed-Systems • Cryptographic • Architectural)
Membrane: Neutral • Non-Activating • Charter-Aligned
Purpose: Establish the NDH-Meta-Systems five-boundary audit baseline and architectural refactoring roadmap for Borealiscodes/memnet, separating deterministic O(log N) routing from local BM25F indexing, upgrading authentication from symmetric HMAC to asymmetric Ed25519 capabilities, and enforcing consent-gated dormancy.
Cryptographic-Anchor: Ed25519 (SHA256:QUOn/esk9dfLFyc/AwFHMIlK5zbI5JOpZTI5vh3y7Dg)
Anchors:
- Borealiscodes/memnet (Research Fork)
- NDH-Meta-Systems (Five-Boundary Methodology & Series 1–2 Audits)
- Constitutional-Organism-Constructor (v2.1 Cryptographic Membrane)
- Author Sovereignty & Collaboration Ecosystem Charter v1.1
- Epistemic Coevolution, Statutory Friction, and Sovereign Edge Governance (DOI: 10.5281/zenodo.24149283)
Non-Activation Clause: This specification is descriptive and architectural only. It does not activate NDH geometry, holonomy engines, constellation routing, lineage engines, or governance altitude. All geometry remains sealed-layer neutral.
Maintainer: Borealis S. Hedling
Location: Dublin, Ireland
Timestamp: 13 November 2026 — 09:16 GMT
Seal: [ N D H • M E M N E T • R E M E D I A T I O N • 0 1 ]
---
