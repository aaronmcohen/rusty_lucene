# Feature Specification: Rust Lucene Scaffolding

## Clarifications

### Session 2026-08-27

<!-- Clarifications recorded during this session -->

- Q: Should the root `Cargo.toml` configure a Cargo workspace (using `[workspace]` members to link sub-crate names), or should it be a single-crate `Cargo.toml` at the root with sub-directories kept separate (not linked via workspace)?  
  A: B – The root `Cargo.toml` will be an initial single-crate `Cargo.toml` (without `[workspace]` members). When sub-crate directories are created later, `[workspace]` members will be added to link them. The overall project structure remains a virtual workspace concept (root `Cargo.toml` linking to sub-crates), but the initial root `Cargo.toml` does not yet specify workspace members.

**Feature Branch**: `001-rust-lucene-scaffolding`

**Created**: 2026-04-27 (actual date from now)

**Status**: Draft → Complete

**Input**: User description: "The project repo should contain a rust Cargo project that is a rust port of lucene core (https://github.com/apache/lucene/tree/main/lucene/core). The scope of this spec is only the project scaffolding. The top level Makefile should contain a target to clone the apache lucene git repository to a `references` folder. Contents of that directory should be ignored by git. The default target of the top level Makefile is help."

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Repository is properly scaffolded with Cargo project structure (Priority: P1)

As a developer starting the Rust port of Lucene, I want the project to have the minimal Rust Cargo structure (Cargo.toml, src/ directory) so I can begin implementing the port without setting up the project from scratch.

**Why this priority**: Starting developers need a working baseline structure to compile and test early; without it they must spend time on setup.

**Independent Test**: Run `cargo build` from the repository root without any manual file creation — it should succeed with a minimal "Hello world"-style build.

**Acceptance Scenarios**:

1. **Given** a fresh clone of the repository, **When** I run `cargo build`, **Then** it builds successfully without requiring additional initialization steps.
2. **Given** a fresh clone of the repository, **When** I check the root directory contents, **Then** it contains at minimum `Cargo.toml` and `src/` directory.

### User Story 2 - Makefile has `help` as default target and `references` target (Priority: P1)

As a developer using the repository, I want the Makefile's default target to be `help` and to have a `references` target that clones the Apache Lucene repository so I can easily get reference material without remembering the exact git command.

**Why this priority**: The default target should clearly inform users what happens by default (`help`), and the `references` target provides essential reference material (`lucene/core`).

**Independent Test**: Run `make` with no arguments — it should output usage instructions; run `make references` — it should clone the Lucene repo to `references/` without interactive prompts.

**Acceptance Scenarios**:

1. **Given** a fresh clone of the repository, **When** I run `make` with no arguments, **Then** it displays a clear "help" message listing available targets (`help`, `references`, `build`, etc.).
2. **Given** a fresh clone of the repository, **When** I run `make references`, **Then** it clones https://github.com/apache/lucene.git (or the specific lucene/core branch/tag) into `references/` directory.

### User Story 3 - `references` directory contents are git-ignored (Priority: P2)

As a developer, I want the `references/` directory (which contains the cloned Lucene source code) to be ignored by git so that the repository remains clean and doesn't bloat unnecessarily with large reference material that shouldn't be part of the actual source code.

**Why this priority**: Reference material is useful but shouldn't be tracked in git; excluding it keeps the repo small and avoids accidental commits of large data.

**Independent Test**: Run `git status` after creating the `references/` directory — it should show the directory as untracked or ignored (not staged), and `git add references` should not be needed for the reference material.

**Acceptance Scenarios**:

1. **Given** a fresh clone with no `references/` yet, **When** I run `make references`, **Then** `references/` is created and `git status` shows it as untracked but not automatically staged.
2. **Given** after `make references`, **When** I check `.gitignore`, **Then** it contains an entry that ignores `references/` and its contents.
3. **Given** after `make references`, **When** I run `git add .` or `git commit`, **Then** `references/` is not included in the commit.

### Edge Cases

- What happens when `make references` is run twice (already exists)? → Run `git pull` (with `--depth 1`) if `references/` is out of date; skip (do nothing) if already up to date. On network failure (e.g., during pull or clone), exit with a clear error message.
- How does system handle network failure during `make references`? → Fail gracefully with a clear error message (e.g., "No internet connection: cannot clone/pull Lucene repository").
- What if the user runs `make references` from an empty directory with no network? → Fail with a helpful message (e.g., "No internet connection: cannot clone Lucene repository; please run with network available or use local reference").

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The repository MUST contain a minimal Rust Cargo project structure at the root (i.e., `Cargo.toml` and `src/` directory). The initial `Cargo.toml` is a single-crate file (without `[workspace]` members). Future sub-crate directories will be linked via `[workspace]` members in the root `Cargo.toml`, making the overall project a virtual workspace.
- **FR-002**: The top-level `Makefile` MUST have `help` as its default target (i.e., `make` without arguments invokes the `help` target).
- **FR-003**: The top-level `Makefile` MUST contain a `references` target that clones the Apache Lucene repository (https://github.com/apache/lucene.git) to a `references/` directory.
- **FR-004**: The `.gitignore` file MUST contain an entry that ignores `references/` and its contents.
- **FR-005**: Running `cargo build` from the repository root MUST succeed without additional setup steps.
- **FR-006**: Running `make references` MUST succeed if network is available and the clone succeeds; it MUST fail with a clear error message if network is unavailable or the clone fails.
- **FR-007**: The `help` target in the Makefile MUST display a clear, concise message listing available targets (`help`, `references`, `build`, etc.).

### Key Entities

- **Repository**: The root directory containing all project files (Cargo.toml, src/, Makefile, .gitignore, etc.)
- **Makefile**: Top-level build/automation file defining targets including `help` (default), `references`, `build`, etc.
- **references/**: Directory that stores the cloned Lucene source code as reference material (git-ignored)
- **.gitignore**: File that lists patterns for files/directories to exclude from git tracking

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Running `make` with no arguments outputs a clear help message listing available targets (verifiable by checking output contains "help", "references", "build" targets).
- **SC-002**: Running `make references` successfully clones the Lucene repository into `references/` in under 60 seconds over typical network (verifiable by checking `references/` directory exists and contains Lucene source).
- **SC-003**: `references/` directory is ignored by git (verifiable by running `git status` showing `references/` as untracked/ignored, and `git add references` is not automatically needed for the reference material).
- **SC-004**: Running `cargo build` succeeds without requiring manual file creation or setup (verifiable by observing successful build output).
- **SC-005**: The Makefile `help` target is the default (`make` without args invokes help) — verifiable by running `make` with no arguments.

### Assumptions

- Target users have internet access available when running `make references` (reference material needs to be cloned)
- Target users have basic CLI tools available (`make`, `git`, `cargo`)
- The Lucene repository will remain available at https://github.com/apache/lucene.git
- Reference material (`references/`) should NOT be included in source commits (hence git-ignored)
- No specific Lucene version is fixed; the `references` target should clone the default branch (main) or could optionally allow specifying a tag/commit

## Notes

- The scope of this spec is ONLY project scaffolding — no actual Lucene port implementation is included
- All requirements are technology-agnostic in wording (e.g., "Makefile", "git", "cargo" used as tools); success criteria are measurable and verifiable
- No [NEEDS CLARIFICATION] markers remain
- See checklist at `specs/001-rust-lucene-scaffolding/checklists/requirements.md` for validation results