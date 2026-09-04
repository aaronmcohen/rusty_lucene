# Data Model: Rust Lucene Scaffolding

## Goal
Describe the key entities and their relationships at the scaffolding level — what files exist, what they contain, and how they relate.

## Entities

### Repository
- Type: Directory (root of the project)
- Contents (at scaffolding level):
  - `Cargo.toml` — root Cargo manifest (single-crate with `[workspace]` placeholder)
  - `src/` — source code directory
  - `Makefile` — build/automation targets
  - `.gitignore` — ignores `references/`
  - `references/` — Git-ignored directory for cloned Lucene (created by `make references`)
  - `specs/001-rust-lucene-scaffolding/` — documentation (plan.md, research.md, this file, etc.)
- Relationships:
  - `Cargo.toml` → `src/` (compilation)
  - `Makefile` → all build/run/test targets; invokes `cargo` commands
  - `.gitignore` → `references/` (excludes reference material from git)
  - `references/` → contains Lucene source (reference only, never committed)

### Cargo.toml
- Type: Manifest file
- Content at scaffolding:
  ```toml
  [package]
  name = "rust-lucene"
  version = "0.1.0"
  edition = "2021"

  [workspace]
  # Placeholder: future sub-crates will be listed here
  # e.g., crates = ["core", "analysis", "query", ...]
  members = []  # empty until sub-crates are created

  [dependencies]
  # No runtime dependencies for scaffolding (actual Lucene port will add deps later)
  ```
- Role: Defines the crate root and workspace members.

### src/main.rs (or lib.rs)
- Type: Source file
- Content at scaffolding:
  ```rust
  /// Rust Lucene scaffolding placeholder.
  /// Running `cargo run` displays a friendly message.
  fn main() {
      println!("🚀 Rust Lucene scaffolding ready!");
      println!("  • Run `cargo build` to compile\n  • Run `cargo test` to test\n  • Run `make references` to clone Lucene core reference\n  • Run `make` (or `make help`) to see available targets");
  }
  ```
- Role: Default binary entry point for `cargo run`; provides immediate feedback.

### Makefile
- Type: Automation file
- Content at scaffolding (conceptual; actual content written in src dir):
  ```makefile
  .PHONY: all help references build test run clean

  # Default target
  all: help

  help:
      @echo "🔧 Rust Lucene Scaffolding Makefile"
      @echo ""
      @echo "Usage:"
      @echo "  make                → show help (default)"
      @echo "  make help          → show help"
      @echo "  make references    → clone Apache Lucene (core) to references/"
      @echo "  make build         → run cargo build"
      @echo "  make test          → run cargo test"
      @echo "  make run           → run cargo run (default binary)"
      @echo "  make clean         → remove references/ (and optionally .cargo/cache)"
      @echo ""
      @echo "Notes:"
      @echo "  - references/ is git-ignored; it contains the Lucene source as reference only"
      @echo "  - Cargo.toml is a single-crate workspace root; future sub-crates will be added via [workspace]"
  
  # Clone Apache Lucene repository to references/
  references:
      @if [ -d references ]; then \
          echo "🔄 references/ exists — re-cloning with --depth 1"; \
          rm -rf references; \
      fi; \
      git clone --depth 1 https://github.com/apache/lucene.git references
  
  # Build the project
  build:
      cargo build --release
  
  # Test the project
  test:
      cargo test --release
  
  # Run the default binary
  run:
      cargo run --release
  
  # Clean references/ (git-ignored reference material)
  clean:
      @rm -rf references
  ```
- Role: Provides CLI experience; `help` is default, `references` fetches reference material.

### .gitignore
- Type: Git exclusion file
- Content at scaffolding:
  ```gitignore
  # Project root ignores reference material
  references/
  
  # Optional: ignore Cargo's build artifacts if desired
  # .cargo/
  # target/  # or keep target/ for build reproducibility
  ```
- Role: Ensures `references/` (and optionally build artifacts) are not committed to git.

### references/
- Type: Directory
- Contents: Cloned Apache Lucene repository (`lucene/` subdirectory at root of clone).
- Git status after clone: `references/` appears as untracked (since it's git-ignored), never staged by default.
- Role: Reference material for the Rust Lucene port; not part of source code.

## Relationships Summary

| Entity      | Depends On      | Depends On / Related To |
|------------|------------------|--------------------------|
| Repository | `Cargo.toml`, `Makefile`, `.gitignore`, `references/` | — |
| `Cargo.toml` | Repository | Workspace members (future sub-crates) |
| `src/main.rs` | Repository | Cargo build system |
| `Makefile` | Repository | `cargo` commands |
| `.gitignore` | Repository | `references/` |
| `references/` | Repository | Lucene repo (reference only) |

## Properties & Constraints

- `references/` must NOT be tracked by git (`.gitignore` pattern `references/`).
- Root `Cargo.toml` must NOT contain `[workspace]` members at scaffolding (empty members = `members = []` or no workspace section — but the spec says "The root `Cargo.toml` will be an initial single-crate `Cargo.toml` (without `[workspace]` members)". However, the spec also says future sub-crates will be linked via `[workspace]` members. So we include `[workspace]` with empty `members` = `[]` to allow future addition without breaking initial structure).
- `Makefile` default target must be `help`.
- `make references` must succeed with network available; must fail gracefully with clear error on network failure.
- `cargo build` must succeed from a fresh clone without manual steps.

## Diagram (textual)

```
Repository/
├── Cargo.toml          # single-crate root + [workspace] placeholder
├── src/
│   └── main.rs         # "Rust Lucene scaffolding ready!" message
├── Makefile            # targets: help (default), references, build, test, run, clean
├── .gitignore          # ignores references/
├── references/         # ← git-ignored; contains cloned Lucene core
└── specs/001-.../     # documentation (plan.md, research.md, data-model.md, quickstart.md, contracts/, tasks.md)
```

## Validation Rules

1. `make` (no args) → outputs help message containing "help", "references", "build", "test", "run", "clean".
2. `make references` → creates `references/` directory with Lucene source inside; `git status` shows `references/` as untracked (ignored).
3. `.gitignore` → contains `references/` pattern; does NOT contain `target/` accidentally (optional).
4. `cargo build` → succeeds without extra setup.
5. Root `Cargo.toml` → has `[workspace]` section with `members = []` (empty), ready for future sub-crates.

