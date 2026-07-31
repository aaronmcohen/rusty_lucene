# Task List for Feature: 001-rusty-lucene-core

## Phase 1: Setup (Project Initialization)
- [X] T001 Create the Rust crate using Cargo (`cargo new rusty_lucene_core --lib`)
- [X] T002 Initialize a Git repository in the project root and commit the initial scaffold.

## Phase 2: Foundational (Blocking Prerequisites for All Stories)
- [X] T003 Add necessary dependencies to `Cargo.toml` *(no runtime deps – adhering to Minimal Dependency Baseline)*
- [X] T004 Create a basic logging setup in `src/main.rs` / `src/lib.rs`.  *(Edit file: src/lib.rs)*

## Phase 3: User Story P1 – “Scaffold the core crate” (Priority P1)
### Tests (Optional – not requested, omitted)
*(No test tasks generated because tests were not explicitly asked for.)*

- [X] T005 [US1] Create a module layout (`src/core/`, `src/util/`).  *(Create directories: src/core/, src/util/)*
- [X] T006 [P] [US1] Add placeholder implementation files:
  - `src/core/mod.rs`
  - `src/util/mod.rs`  *(Edit/create files: src/core/mod.rs, src/util/mod.rs)*
- [X] T007 [US1] Write a simple “Hello‑World” function in `src/lib.rs` to verify the crate compiles.  *(Edit file: src/lib.rs)*
- [X] T008 [US1] Add unit test for the placeholder function (if desired).  *(Create/edit file: tests/placeholder_test.rs)*

## Final Phase 4: Polish & Cross‑Cutting Concerns
- [X] T009 Update `README.md` with project description, build instructions, and usage notes.  *(Edit/create file: README.md)*
- [X] T010 Run the full test suite (`cargo test`) to ensure everything passes.  *(Bash command: cargo test)*

## Phase 5: Convergence
- [X] T025 Added append‑only guard for modifications to `tasks.md` per FR‑08
