---
description: "Task list for Rust Lucene Scaffolding feature"
---

# Tasks: 001-rust-lucene-scaffolding

**Input**: Design documents from `/specs/001-rust-lucene-scaffolding/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create project structure per implementation plan
  - Create `tests/` directory with subdirectories `contract/`, `integration/`, `unit/`
  - File path: `/Users/aaronmcohen/Projects/rusty_lucene/tests/contract/`, `/Users/aaronmcohen/Projects/rusty_lucene/tests/integration/`, `/Users/aaronmcohen/Projects/rusty_lucene/tests/unit/`

- [x] T002 Initialize Cargo.toml with [workspace] placeholder
  - File path: `/Users/aaronmcohen/Projects/rusty_lucene/Cargo.toml`
  - Add `[workspace]` placeholder (e.g., `[workspace] = ["crates/*"]`)

- [x] T003 [P] Create .gitignore with references/ ignore
  - File path: `/Users/aaronmcohen/Projects/rusty_lucene/.gitignore`
  - Add `references/` entry

- [x] T004 [P] Create Makefile with help default target
  - File path: `/Users/aaronmcohen/Projects/rusty_lucene/Makefile`
  - Set `.DEFAULT_GOAL ?= help` and include `help` target that lists available targets

- [x] T005 [P] Create tests/ directory structure
   - File path: `/Users/aaronmcohen/Projects/rusty_lucene/tests/`
   - Create subdirectories `contract/`, `integration/`, `unit/`

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 [P] Add `references` target to Makefile that clones Lucene with depth=1
  - File path: `/Users/aaronmcohen/Projects/rusty_lucene/Makefile`
  - Add `references:` target that runs `git clone --depth 1 https://github.com/apache/lucene.git references`

- [x] T007 [P] Verify `make references` works
  - Command: `make references`
  - Verification: `references/` directory exists and contains Lucene source

## Phase 3: User Story 1 - Repository is properly scaffolded (Priority: P1) 🎯 MVP

**Goal**: Minimal Rust Cargo project structure (Cargo.toml, src/) so developers can begin implementing the port without setting up the project from scratch.

**Independent Test**: Run `cargo build` from the repository root without any manual file creation — it should succeed with a minimal "Hello world"-style build.

### Implementation for User Story 1

- [x] T008 [P] [US1] Create src/lib/ directory structure
  - File path: `/Users/aaronmcohen/Projects/rusty_lucene/src/lib/`
  - Create directory

- [x] T009 [US1] Create src/main.rs or src/lib.rs minimal "Hello world"-style build
  - File path: `/Users/aaronmcohen/Projects/rusty_lucene/src/main.rs` or `/Users/aaronmcohen/Projects/rusty_lucene/src/lib.rs`
  - Content: minimal Rust code that compiles and runs (e.g., `fn main() { println!("Hello, world!"); }`)

- [x] T012 [US1] Verify `cargo build` succeeds
  - Command: `cargo build` from repository root
  - Verification: Build succeeds without errors

## Phase 4: User Story 2 - Makefile has `help` as default target and `references` target (Priority: P1)

**Goal**: Makefile's default target is `help` and it has a `references` target that clones the Apache Lucene repository.

**Independent Test**: Run `make` with no arguments — it should output usage instructions; run `make references` — it should clone the Lucene repo to `references/` without interactive prompts.

### Implementation for User Story 2

- [x] T013 [P] [US2] Verify `make` outputs help message
  - Command: `make`
  - Verification: Output contains "Available targets:" with "help", "references", "build", etc.

- [x] T014 [US2] Verify `make references` clones Lucene successfully
  - Command: `make references`
  - Verification: `references/` directory exists and contains Lucene source

## Phase 5: User Story 3 - `references` directory contents are git-ignored (Priority: P2)

**Goal**: The `references/` directory (which contains the cloned Lucene source code) is ignored by git so that the repository remains clean and doesn't bloat unnecessarily with large reference material that shouldn't be part of the actual source code.

**Independent Test**: Run `git status` after creating the `references/` directory — it should show it as untracked but not automatically staged.

### Implementation for User Story 3

- [x] T015 [P] [US3] Verify `.gitignore` contains `references/` pattern
  - File path: `/Users/aaronmcohen/Projects/rusty_lucene/.gitignore`
  - Verification: Contains `references/` entry

- [x] T016 [US3] Verify `git status` after `make references` shows references/ as untracked
  - Command: `make references` then `git status`
  - Verification: `references/` appears as untracked in git status

- [x] T017 [US3] Verify `git add references` does NOT automatically stage references/
  - Command: `git add references`
  - Verification: `git status` shows `references/` as added (not committed), and no automatic commit

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T020 [P] Documentation updates in quickstart.md
  - File path: `/Users/aaronmcohen/Projects/rusty_lucene/specs/001-rust-lucene-scaffolding/quickstart.md`
  - Content: Add quickstart instructions showing how to run `make help` and `make references`

- [x] T021 [P] Code cleanup and refactoring
  - File path: `/Users/aaronmcohen/Projects/rusty_lucene/Makefile`
  - Verification: Makefile is clean, consistent formatting, no redundant checks

- [x] T022 [P] Run quickstart.md validation
  - Command: Run `make help` and verify output is clear and concise
  - Verification: Help message lists all targets (`help`, `references`, `build`, etc.)

### Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if team capacity allows)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

### Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together (if tests requested):
Task: "Contract test for [endpoint] in tests/contract/test_[name].py"
Task: "Integration test for [user journey] in tests/integration/test_[name].py"

# Launch all models for User Story 1 together:
Task: "Create src/lib/ directory structure"
Task: "Create src/main.rs minimal 'Hello world' build"
```

### Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

### Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
