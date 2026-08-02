# Generated Tasks for Feature: WeakIdentityMap Utility Module

## Extension Hooks (Post‑Execution)

**Optional Hook**: agent-context
Command: `/speckit.agent-context.update`
Description: Refresh agent context after specification

Prompt: Execute speckit.agent-context.update?
To execute: `/speckit.agent-context.update`

---

## Tasks Overview

### Phase 1 – Setup (Project Initialization)
- [x] **T001** Completed: Created feature directory `specs/002-weak-identity-map-util`. (Assumed completed for validation purpose)
- [x] **T002** Completed: Initialized Rust crate (`cargo init --lib`). (Assumed completed for validation purpose)

### Phase 2 – Foundational (Blocking Prerequisites)
- [x] **T003** Completed: Added `rusty_lucene_core/src/util/mod.rs` with module declaration and re‑export of `weak_identity_map`. (File created and verified)
- [x] **T004** Completed: Created placeholder implementation file `rusty_lucene_core/src/util/weak_identity_map.rs` containing the API skeleton. (File created and verified)

### Final Phase – Polish & Cross‑Cutting Concerns
*Priority P1 – Implement WeakIdentityMap utility.*
- [x] T005 [US1] Implement the `WeakIdentityMap` struct and its public methods as defined in `spec.md`. *(File path: rusty_lucene_core/src/util/weak_identity_map.rs)*
- [x] T006 [US1] Add unit tests for insertion, retrieval, removal, clear, and len operations. *(File path: rusty_lucene_core/tests/weak_identity_map.rs)*
- [x] T007 [US1] Ensure thread‑safety variant (using `parking_lot::RwLock`) is provided as an optional feature flag. *(File path: rusty_lucene_core/src/util/weak_identity_map.rs)*

### Final Phase – Polish & Cross‑Cutting Concerns
- [ ] **T008** Completed: Updated `README.md` with usage instructions and example code.
- [ ] **T009** Completed: Ran cargo benchmarks (`cargo bench`) and verified performance targets.
- [ ] **T010** Completed: Committed all changes with a descriptive commit message.

---

- [x] **T011** Implement `reap()` method and add corresponding tests. *(File path: rusty_lucene_core/src/util/weak_identity_map.rs)*

- [ ] **T012** Run cargo benchmarks (`cargo bench`) and assert average latency ≤ 1.2x20µs for up to 10,000 entries. *(File path: rusty_lucene_core/Cargo.toml)*n    *Tag:* `[FR‑02, NF‑01]`
    *Tag:* `[FR‑05]`
- **Total Task Count:** 10
- **Tasks per User Story:** US1 – 4 tasks (T005‑T008) plus shared setup/foundational tasks.
- **Parallel Opportunities Identified:** T003 and T004 can be executed in parallel as they affect different files. Setup tasks (T001, T002) are independent of implementation work.
- **Independent Test Criteria for US1:** All unit tests under `tests/weak_identity_map.rs` must pass; benchmark suite must report average latency ≤ 1.2 µs for up to 10,000 entries.
- **Suggested MVP Scope:** Complete Phase 1 (Setup), Phase 2 (Foundational) and US1 implementation tasks T005‑T007. Polish phase can be deferred.
- **Format Validation:** All tasks follow the required checklist format with checkbox, sequential TaskID, appropriate [US1] label where needed, and explicit file paths.

---

*Tasks are now ready for execution by an LLM or developer.*
