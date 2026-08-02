# Plan for WeakIdentityMap Implementation

## Milestones
1. **Design** – Finalize API surface, thread‑safety strategy, and reclamation semantics.
2. **Implementation** – Add `src/util/weak_identity_map.rs` with the public methods defined in *spec.md*.
3. **Testing** – Write unit tests covering:
   - Weak key eviction after drop
   - Concurrent access correctness (using mutex/RwLock)
   - Clear and explicit `reap()` behavior
4. **Benchmarking** – Verify O(1) average‑case performance for up to 10,000 entries.
5. **Documentation** – Update *spec.md* with missing non‑functional details (performance benchmarks, error handling, sizing constraints, OS assumptions).
6. **Review & Merge** – Peer review and merge into `main` branch.

## Dependencies
- Rust standard library (`std::rc`, `std::sync`).
- Optional third‑party crates (`parking_lot`) gated behind feature flags.

## Risks & Mitigations
| Risk | Impact | Mitigation |
|------|--------|------------|
| Unbounded growth if weak keys never get collected. | Memory leak. | Implement "reap on read" and expose explicit `reap()`; add tests that verify entry count drops after dropping strong refs. |
| Deadlock in concurrent mode. | Runtime failure. | Use well‑tested synchronization primitives (`Mutex`/`RwLock`) and write stress‑test for lock acquisition failures (see *spec.md* error handling section). |
| Missing OS constraints documentation. | Unexpected behavior on certain platforms. | Add a Non‑Functional Requirements subsection detailing memory model assumptions and any platform‑specific limits. |

## Open Questions Resolved in spec.md Updates
- **Error Handling & Failure Scenarios** – Added section describing out‑of‑memory conditions, hash collision handling, lock acquisition failures.
- **Sizing Constraints** – Documented optional capacity limits and graceful degradation behavior.
- **Performance Benchmarks** – Specified target O(1) lookup time and provided a benchmarking approach.
- **Zero‑Entry Edge Case** – Clarified that `get`, `remove`, and `len` return safe defaults when the map is empty.
- **Lock Acquisition Failure Recovery** – Defined fallback (retry or error propagation) in concurrent mode.
- **OS‑Level Assumptions** – Noted reliance on typical 64‑bit address space and no special OS calls.

*Plan last updated: 2025‑08‑31.*
