# Rust Core Refactoring Plan: Deterministic Routing, Ed25519 Identity & Consent Membranes (v1.0)
**Target Repository:** `Borealiscodes/memnet-specimen`  
**Methodology:** Five-Boundary Separation of Concerns  
**Canonical Reference:** *Epistemic Coevolution, Statutory Friction, and Sovereign Edge Governance* (`DOI: 10.5281/zenodo.24149283`)  

---

## 1. Objective
Refactor the Rust core of `memnet-specimen` to maintain deterministic property tests and core mathematical kernels while isolating transport from lexical search, migrating from symmetric HMAC authentication to `Ed25519` asymmetric keys, and enforcing compile-time consent states.

---

## 2. Target Rust Module Architecture (`src/`)

| Module Path | Action | Core Responsibility & Invariant |
| :--- | :--- | :--- |
| `src/complex_ternary.rs` | **Keep (Untouched)** | Retain the 9-state complex ternary oracle and all deterministic property tests. |
| `src/framing.rs` | **Extract & Keep** | Isolate `PacketBatcher` and MTU profiles as a pure framing layer. |
| `src/identity.rs` | **Replace** | Replace shared-secret HMAC with `Ed25519` public-key verification and SHA-256 node IDs. |
| `src/membrane.rs` | **Replace** | Enforce compile-time phantom types for peer consent and pull-based wake sync. |
| `src/routing.rs` | **Add** | Implement monotonic 256-bit XOR Kademlia DHT routing over SHA-256 digests. |
| `src/retrieval.rs` | **Scope Locally** | Restrict `BM25F + RRF` ranking strictly to a local, opt-in application index. |
| `src/bin/memnet.rs` | **Sanitize & Refactor** | Strip hardcoded personal device identifiers and use RFC 5737 test addresses. |

---

## 3. Four-Phase Implementation Roadmap

* **Phase 1: OpSec Sanitization & Kernel Isolation:** Audit fixtures to remove private IP/device data and extract `PacketBatcher` into `src/framing.rs`.
* **Phase 2: Asymmetric `Ed25519` Identity (`src/identity.rs`):** Transition Node IDs and capability grants to public-key cryptography.
* **Phase 3: Compile-Time Consent Membrane (`src/membrane.rs`):** Remove unsolicited push beacons and default peer sessions to a dormant state requiring explicit consent.
* **Phase 4: Deterministic XOR Routing & Local Index (`src/routing.rs` & `src/retrieval.rs`):** Separate DHT routing mechanics from local search indexing.

---

# 🧾 **Provenance Footer — Rust Core Refactoring Plan (v1.0)**

For complete metadata and documentation, consult the repository specification files.
