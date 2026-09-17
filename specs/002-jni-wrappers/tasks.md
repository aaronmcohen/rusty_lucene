# Tasks: JNI Wrappers

**Input**: Design documents from `/specs/002-jni-wrappers/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: The examples below include test tasks. Tests are OPTIONAL - only include them if explicitly requested in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3) — Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root

<!--
  ============================================================================
  IMPORTANT: The tasks below are SAMPLE TASKS for illustration purposes only.

  The /speckit.tasks command MUST replace these with actual tasks based on:
  - User stories from spec.md (with their priorities P1, P2, P3...)
  - Feature requirements from plan.md
  - Entities from data-model.md
  - Endpoints from contracts/

  Tasks MUST be organized by user story so each story can be:
  - Implemented independently
  - Tested independently
  - Delivered as an MVP increment

  DO NOT keep these sample tasks in the generated tasks.md file.
  ============================================================================
-->

## Phase 0: Research (AM1/AM2 Resolution)

**Purpose**: Resolve AM1 (only create `@jni/<crate-name>/` for crates with JNI-suitable exports) and AM2 (minimal `jni-update` script) decisions

- [ ] T000 Resolve AM1: `@jni/<crate-name>/` directory is created only for Rust sub-crates that export at least one native function suitable for JNI mapping. Crates with no JNI-suitable exports (e.g., complex enum-based functions) are skipped — no wrapper directory/project is created for them.
- [ ] T001 Resolve AM2: Create minimal `jni-update` script that runs after `cargo build --release` and regenerates only the JNI wrapper project for applicable crates (crates with JNI-suitable exports). The script should:
  - Scan `@crates/` for `#[no_mangle] pub extern "C"` native exports
  - Determine which crates have JNI-suitable exports (simple whitelist: basic types like `i8,i16,i32,i64`, `f32,f64`, `*const c_char`, `&str`/`String`, pointers without complex data payloads)
  - Skip crates without JNI-suitable exports (e.g., `get_complex_enum` with enum payload — documented as JNI-incompatible)
  - Regenerate `@jni/<crate-name>/` structure (cpp glue + java class + contract) only for applicable crates
- [ ] T002 Create `specs/002-jni-wrappers/contracts/jni-contract.md` with method signatures from Rust exports; document JNI-incompatible APIs
  - List methods: `search`, `rank` as JNI-suitable; `get_complex_enum` as JNI-incompatible (document reason and Java fallback approach)
- [ ] T003 Create `tasks.md` (this file)
- [ ] T004 [P] Add `jni-*` targets (`jni-build`, `jni-test`, `jni-clean`) to top-level `Makefile` (if not already present)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Verify Rust native exports
  - Run `cargo build --release` to produce `target/release/librusty_lucene_core.{so/dylib}`
  - Inspect `#[no_mangle] pub extern "C"` functions in `rusty-lucene-core` (e.g., `search`, `rank`, `get_complex_enum`)
- [ ] T005 Confirm JNI linking works
  - Run `./gradlew nativeCompile` — should succeed without linker errors
  - On macOS: `-dynamiclib`; on Linux: `-shared`; verify the linker can find `librusty_lucene_core` from `target/release/`

---

## Phase 3: User Story 1 - Create JNI wrapper for a Rust crate (Priority: P1)

**Goal**: Generate Java JNI wrapper project under `@jni/` that links against Rust native library

**Independent Test**: Given `@crates/` contains N Rust sub-crates, when the JNI wrapper generation tool runs (or on crate addition), then exactly the applicable crates (those with JNI-suitable native exports) have `@jni/<crate-name>/` wrapper projects created under `@jni/` (no `-jni` suffix; crates with no JNI-suitable exports are skipped).

### Tests for User Story 1 (optional)

- [ ] T007 Contract test for JNI method signature — verify `jni-contract.md` matches actual JNI glue (`rusty_lucene_jni.cpp` and `RustyLuceneJni.java`)

### Implementation for User Story 1

- [ ] T008 [P] [US1] Ensure `jni/rusty-lucene-core/src/main/cpp/rusty_lucene_jni.cpp` exists with JNI glue
  - Include `jni.h`, forward declarations for Rust exported functions
  - Implement `Java_rusty_lucene_jni_search`, `Java_rusty_lucene_jni_rank`, etc., converting params to JNI types
- [ ] T009 [US1] Ensure `jni/rusty-lucene-core/src/main/java/com/example/rclucene/jni/RustyLuceneJni.java` exists with native methods
  - Public native methods (e.g., `public native int search(String query);`)
  - Fallback methods for JNI-incompatible APIs (e.g., `public String getComplexEnum(int id);`)
- [ ] T010 [US1] Populate `specs/002-jni-wrappers/contracts/jni-contract.md` with actual method signatures from Rust exports

**Checkpoint**: JNI wrapper project `jni/rusty-lucene-core/` exists with cpp + java, `contracts/jni-contract.md` is populated, and `README.md` has validation instructions.

---

## Phase 4: User Story 2 - Build and test JNI wrapper integration (Priority: P2)

**Goal**: Build both Rust native library and JNI wrapper, then verify Java can call into Rust correctly

**Independent Test**: Build Rust native library + JNI wrapper, run simple Java test that calls a JNI method, and observe correct Rust response

### Tests for User Story 2 (optional)

- [ ] T012 [P] Native build test — `./gradlew nativeCompile` succeeds
- [ ] T013 [P] JNI integration test — `./gradlew nativeTest` passes (loads native library and exercises at least one JNI method)

### Implementation for User Story 2

- [ ] T014 [P] [US2] Verify `jni-build` Makefile target works
  - `jni-clean`: remove `build/native/*.dylib/*.so` and `build/libs/*`
  - `jni-build`: `jni-clean`, `cargo build --release`, `./gradlew nativeCompile`
  - Note: build only `@jni/<crate-name>/` projects for applicable crates (skip crates with no JNI-suitable exports)
- [ ] T015 [P] [US2] Verify `jni-test` Makefile target works
  - `jni-test`: `jni-build`, then `./gradlew nativeTest`
  - Note: test only applicable JNI wrapper projects (skip non-applicable crates)
- [ ] T016 [US2] Create simple Java test class (in `src/test/java/`) that calls a JNI method

**Checkpoint**: Running `make jni-build` builds Rust + JNI, and `make jni-test` (or `./gradlew nativeTest`) successfully loads the native library and passes at least one test.

---

## Phase 5: User Story 3 - Maintain consistency when Rust crates change (Priority: P3)

**Goal**: Automatically regenerate or minimally update the JNI wrapper when a Rust sub-crate is modified

**Independent Test**: Modify a Rust sub-crate's native export (add/modify/remove a function), then regenerate the JNI wrapper, and verify the Java wrapper correctly reflects the change

### Tests for User Story 3 (optional)

- [ ] T017 [P] Regression test — change Rust function signature, rebuild, verify JNI glue + Java class pick it up without manual edit of C++ glue

### Implementation for User Story 3

- [ ] T018 [P] [US3] Create minimal `jni-update` script that regenerates only applicable JNI wrappers
  - Script scans `@crates/` for `#[no_mangle] pub extern "C"` exports
  - Identifies crates with JNI-suitable exports (e.g., `search`, `rank`) and skips crates with JNI-incompatible exports (e.g., `get_complex_enum`)
  - Regenerates `@jni/<crate-name>/` structure (cpp glue + java class) only for applicable crates
  - Fallback: document minimal manual update steps in `README.md` (only update applicable crates; skip non-applicable ones)
- [ ] T019 [US3] Document update flow in `README.md`
  - "When adding/modifying a Rust native function that is JNI-suitable (e.g., `search`, `rank`): 1. Run `cargo build --release`; 2. Update `jni/rusty-lucene-core/src/main/cpp/rusty_lucene_jni.cpp`; 3. Update `src/main/java/com/example/rclucene/jni/RustyLuceneJni.java`; 4. Run `./gradlew nativeCompile` and `./gradlew nativeTest`. JNI-incompatible functions (e.g., `get_complex_enum`) are documented as such — no wrapper directory/project is created and no manual glue update is needed."

**Checkpoint**: JNI wrapper can be updated with minimal manual steps after a Rust native export change.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, validation, and cleanup

- [ ] T020 [P] Create `specs/002-jni-wrappers/research.md` — resolve "NEEDS CLARIFICATION" items
- [ ] T021 [P] Update `README.md` with concise quick-start
  - Prerequisites: Rust toolchain + JDK 11+, GCC/Clang
  - Setup: `cargo build --release`, `cd jni/rusty-lucene-core && ./gradlew nativeCompile` (only applicable crates have a wrapper directory)
  - Test: `./gradlew nativeTest` (sets DYLD_LIBRARY_PATH/LD_LIBRARY_PATH automatically)
  - Expected: all tests pass, native library loads
- [ ] T022 [P] Verify `jni-*` targets (`jni-build`, `jni-test`, `jni-clean`) in Makefile work correctly
- [ ] T023 [P] Clean up: `make jni-clean` removes `build/native/*.dylib/*.so` and `build/libs/*`
- [ ] T024 Run validation: `make jni-build && make jni-test` → all succeed

---

## Dependencies & Execution Order

- **Phase 0: Research (AM1/AM2 Resolution)**: No dependencies — start immediately
- **Foundational (Phase 1)**: Depends on Phase 0 Research completion — blocks all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational Phase 1 completion
  - US1 → complete after Foundational + Phase 0
  - US2 → complete after Foundational + Phase 0
  - US3 → complete after Foundational + Phase 0
- **Polish (Phase 6)**: Depends on all user stories complete

Within each User Story:
- Tests (if included) MUST be written and FAIL before implementation
- Contracts/glue before Java class
- Story complete before moving to next priority

### Parallel Opportunities

- All Research tasks (T000–T003) marked [P] can run in parallel
- All Foundational tasks (T004–T005) marked [P] can run in parallel
- Once Foundational completes, user stories can start in parallel (if team capacity allows)
- Tests for a story marked [P] can run in parallel
- Contracts within a story marked [P] can run in parallel

### Parallel Example: User Story 1

```bash
# Contract test
./gradlew :specifications:002-jni-wrappers:classes  # verify contracts match reality

# JNI glue
./gradlew nativeCompile  # build C++ glue

# Java wrapper
./gradlew compileJava  # compile Java class
```

### Implementation Strategy

#### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (create contracts/ + README + Makefile targets)
2. Complete Phase 2: Foundational (verify Rust native exports, confirm linking works)
3. Complete Phase 3: User Story 1 (JNI glue + Java class + contracts)
4. **STOP and VALIDATE**: Run `make jni-build && make jni-test` — all succeed
5. Deploy/demo if ready

#### Incremental Delivery

1. Setup + Foundational → foundation ready
2. US1 → test independently → MVP!
3. US2 → test independently → build + test workflow works
4. US3 → update flow works with minimal manual steps
5. Polish → docs, validation, cleanups

#### Parallel Team Strategy

1. Team completes Setup + Foundational together
2. Dev A: User Story 1 (glue + Java class + contracts)
3. Dev B: User Story 2 (Makefile targets + integration tests)
4. Dev C: User Story 3 (update script + documentation)
5. All stories integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same-file conflicts, cross-story dependencies that break independence

---

## Validation

After `make jni-build` and `make jni-test` succeed, the feature is complete:
- JNI native library links against Rust's `librusty_lucene_core`
- Java `RustyLuceneJni` can call Rust functions via JNI
- `README.md` quick-start works
- `contracts/jni-contract.md` documents method signatures and JNI-incompatible APIs
- `tasks.md` exists with proper task structure

✅ **Done When**: `research.md`, `contracts/jni-contract.md`, `tasks.md`, `README.md`, and project structure complete, and `make jni-build && make jni-test` passes.
