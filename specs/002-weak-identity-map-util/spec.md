# Feature: WeakIdentityMap Utility Module for rusty_lucene_core

## Overview
Add a `util` module to the `@rusty_lucene_core/` Cargo package that provides a Rust implementation of **WeakIdentityMap**. 
The behavior should mirror the Java reference class located at `​@references/lucene/lucene/core/src/java/org/apache/lucene/util/WeakIdentityMap.java`, exposing an API for identity‑based weak key storage.

## Existing Codebase Context
The repository already contains a `src/util` directory (`mod.rs`). Therefore the new implementation will be added as a submodule within this module hierarchy rather than creating a duplicate top‑level package.

## User Scenarios & Acceptance Criteria
| Scenario | Description | Success Criterion |
|----------|-------------|-------------------|
| **Cache Integration** | A component uses `WeakIdentityMap` to cache objects identified by reference equality, allowing cached entries to be reclaimed when the referencing object is dropped. | Cached values are automatically removed after all strong references to the key are released, without manual cleanup. |
| **Thread‑Safe Operations** | Multiple threads concurrently call `get`, `put`, and `remove`. | All operations remain correct (no data races) and performance degrades gracefully under load. |
| **Clear Operation** | Explicitly clear the map during application shutdown or memory pressure events. | `clear()` empties the map and releases all held references immediately. |

## Functional Requirements
1. **Module Structure**  
   - Add a new file `src/util/weak_identity_map.rs` inside the existing `src/util` module of the `rusty_lucene_core` crate.
2. **API Surface** (public items)
    ```rust
    pub struct WeakIdentityMap<K, V> { /* … */ }
    impl<K: Eq + Hash + std::marker::Unsize<Any>, V> WeakIdentityMap<K, V> {
        /// Constructs a new map with reference‑equality semantics.
        pub fn new() -> Self { ... }

        /// Inserts a value associated with the given key (reference).  
        /// Returns the previous value if any.
        pub fn insert(&mut self, key: K, value: V) -> Option<V> { ... }

        /// Retrieves the value for the exact reference `key`.  
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
            where Q: ?Sized + std::borrow::Borrow<K> + Eq + std::hash::Hash,
                  K: std::marker::Unsize<Q>,
        { ... }

        /// Removes the entry identified by `key`.  
        pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
            where Q: ?Sized + std::borrow::Borrow<K> + Eq + std::hash::Hash,
                  K: std::marker::Unsize<Q>,
        { ... }

        /// Clears all entries and frees reclaimed memory.  
        pub fn clear(&mut self) { ... }

        /// Returns the current number of live entries (keys not yet GCed).  
        pub fn len(&self) -> usize { ... }
    }
    ```
3. **Identity‑Equality Semantics**  
   - Keys are compared using reference equality (`*key1 == *key2`).  
   - The implementation must behave like Java's `WeakIdentityMap`: weak keys, strong values.
4. **Thread Safety**  
   - Provide a thread‑safe variant via interior mutability (e.g., `parking_lot::RwLock` or `std::sync::Mutex`) if the caller requires concurrent access; otherwise expose an unsync version with clear documentation.
5. **Reclamation Behavior**  
   - Keys that are no longer strongly referenced should be automatically removed on the next call to any public method ("reap on read").  
   - Offer an explicit `reap()` method for manual cleanup if needed.
6. **Testing**  
   - Unit tests covering insertion/retrieval of weak keys, automatic eviction after dropping references, concurrent‑access correctness, and that `clear()` zeroes the size.

## Non‑Functional Requirements
- **Performance:** Average O(1) for `insert`, `get`, `remove`, `len`.
- **Memory Overhead:** Proportional to number of live entries; no unbounded growth.
- **Safety:** No unchecked unsafe code beyond required interior‑mutability wrappers.

## Additional Non‑Functional Requirements (new)
### Error Handling & Failure Scenarios
- The implementation must gracefully handle out‑of‑memory conditions by propagating `std::io::ErrorKind::OutOfMemory` to callers of mutating methods (`insert`, `clear`).
- Hash collisions are resolved using a chaining strategy; excessive chain length triggers a rehash operation that doubles the underlying table size.
- In concurrent mode, if lock acquisition fails after three retries (e.g., due to thread panic), the method returns an `Err(LockAcquireError)` enum variant indicating temporary failure; callers may retry or fallback to single‑threaded usage.

### Sizing Constraints & Capacity Limits
- An optional **capacity hint** can be supplied at construction (`new_with_capacity(hint: usize)`) which sets the initial table size. If omitted, a default of 64 slots is used.
- When the slot utilization exceeds 75%, an automatic rehash expands the table by 1.5× to maintain O(1) amortized cost while limiting memory bloat.
- Users may impose hard caps via a `set_max_capacity(max: usize)` method; exceeding this cap results in rejection of new inserts with `Err(MaxCapacityExceeded)`.

### Performance Benchmarks (Quantified)
- Target average latency for `insert`, `get`, and `remove` must not exceed **1.2 µs** on a typical x86‑84 machine when the map holds up to **10,000** entries.
- Benchmark suite (`cargo bench`) will include stress tests covering 100k inserts/removes under concurrent load (32 threads) and verify that latency remains below **2.5 µs** with < 1% overhead from synchronization primitives.

### Zero‑Entry Edge Case Handling
- All read‑only methods (`get`, `len`, `reap`) safely handle an empty map: `get` returns `None`; `len` returns `0`; `reap()` performs no work and returns `Ok(())`.
- Mutating methods (`insert`, `remove`, `clear`) also succeed without error when the map is empty, preserving a deterministic API surface.

### OS‑Level Assumptions & Constraints
- The implementation assumes a standard 64‑bit Unix/Linux or Windows environment with typical address space limits (e.g., ≤ 256 TB virtual memory). No platform‑specific syscalls are used; therefore the map should operate identically across supported OSes.
- Reliance on Rust's `std::rc::Weak` and `std::sync::{Arc, Mutex, RwLock}` implies that behavior follows the language specification regarding reference counting and atomic operations, which are portable to all targets where these types are stable.

## Assumptions (existing)
1. Rust's `Rc`/`Arc` with weak references can emulate the desired semantics.
2. Users will use the unsync version for single‑threaded contexts and a locked variant when needed.
3. No external crate is mandated; optional third‑party crates must be behind feature flags.

## Success Criteria (Technology‑Agnostic)
- **Correctness:** All acceptance scenarios pass with 100 % test coverage of the public API.
- **Efficiency:** Operations complete within 1 µs for up to 10,000 entries on typical hardware.
- **Reliability:** No memory leaks; values are reclaimed after all strong references drop (verified via heap snapshot).

## Clarifications
### Session 2025‑08‑31
- Q: Given that `src/util` already exists, should the new functionality be placed as a submodule under this existing module hierarchy? **A:** Yes – add `weak_identity_map.rs` inside `src/util` and expose it via the existing `mod.rs`.

- **Performance:** Average O(1) for `insert`, `get`, `remove`, `len`.  
- **Memory Overhead:** Proportional to number of live entries; no unbounded growth.  
- **Safety:** No unchecked unsafe code beyond required interior‑mutability wrappers.

## Assumptions
1. Rust's `Rc`/`Arc` with weak references can emulate the desired semantics.  
2. Users will use the unsync version for single‑threaded contexts and a locked variant when needed.  
3. No external crate is mandated; optional third‑party crates must be behind feature flags.

## Success Criteria (Technology‑Agnostic)
- **Correctness:** All acceptance scenarios pass with 100 % test coverage of the public API.  
- **Efficiency:** Operations complete within 1 µs for up to 10,000 entries on typical hardware.  
- **Reliability:** No memory leaks; values are reclaimed after all strong references drop (verified via heap snapshot).

## Clarifications
### Session 2025-08-31
- Q: Are performance benchmarks still required? **A:** No – they are out of scope as per user input.

- Q: Given that `src/util` already exists, should the new functionality be placed as a submodule under this existing module hierarchy? **A:** Yes – add `weak_identity_map.rs` inside `src/util` and expose it via the existing `mod.rs`.
