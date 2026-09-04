# Phase 0 Research: Rust Lucene Scaffolding

## Goal
Research minimal Rust Cargo scaffolding with a Makefile that defaults to `help` and has a `references` target cloning Apache Lucene, plus `.gitignore` that ignores `references/`. Root `Cargo.toml` is single-crate with `[workspace]` placeholder.

## Key Findings

### 1. Rust Cargo Minimal Structure
- A minimal Cargo project requires `Cargo.toml` and `src/` (with either `src/main.rs` or `src/lib.rs`).
- Root `Cargo.toml` can be a single-crate file without `[workspace]` initially. Workspace support is added via `[workspace]` members when sub-crates are created.
- `src/lib/` or `src/` can contain placeholder `lib.rs` with a simple function or empty struct for compilation.

### 2. Makefile for CLI Experience
- Default target: `help` — displays usage message listing targets.
- Target `references`: clones https://github.com/apache/lucene.git to `references/` directory using `git clone --depth 1`.
- Target `build`: runs `cargo build` (default Cargo build).
- Target `test`: runs `cargo test` (default Cargo test).
- Target `clean`: optional, removes `references/` if present.

### 3. `.gitignore` for References
- Add `references/` pattern to `.gitignore` so the cloned Lucene repo is not tracked.

### 4. `make references` Behavior on Re-run
- Use `git clone --depth 1 --branch main https://github.com/apache/lucene.git references` or `git clone --depth 1 https://github.com/apache/lucene.git references` with `--config remote.origin.url` to handle existing directory gracefully.
- Better: check if `references/` exists and is a git repo; if it exists, run `git fetch --depth 1` or `git pull` to update, rather than re-cloning. `--no-checkout` with `git clone` + `--progress` might be tricky. Simpler: remove `references/` then clone fresh, or use `git clone --depth 1` which safely handles existing directory by updating.
- `git clone --depth 1` automatically fails if directory exists and is not empty; better approach: `git clone --depth 1 --config remote.origin.url=https://github.com/apache/lucene.git --config clone.filter.blob=none references` won't work for update. Use `git fetch --depth 1` if directory exists and is a git repo: `if [ -d references ]; then cd references && git fetch --depth 1; cd ..; fi && git clone --depth 1 https://github.com/apache/lucene.git references`.
- Simpler working approach: `make references` does: `if [ -d references ]; then rm -rf references; fi && git clone --depth 1 https://github.com/apache/lucene.git references`. This re-clones on every run, which is simple and safe (fast with --depth 1).

### 5. Measurable Outcomes Verification
- `make` (no args) → outputs help message.
- `make references` → creates `references/` with Lucene source.
- `git status` after `make references` → shows `references/` as untracked (ignored but not staged).
- `cargo build` → succeeds without extra steps.

## Open Questions

- Should `references/` target allow specifying a Lucene tag/commit via an optional `LUCENE_TAG` variable? → Not required by spec; spec says "clones the Apache Lucene repository" (default branch/tag). We'll clone `main` by default, allowing `make references LUCENE_TAG=v8.12.0` for optional tag support.
- Should the Makefile include a `check` target that verifies `references/` is git-ignored after clone? → No, `.gitignore` is static; verification happens via `git status` test.
- Should `src/` contain a `main.rs` or `lib.rs`? → `src/lib/lib.rs` with `pub fn hello() { println!("hello"); }` is simplest. `src/` could also just have `src/main.rs` with `fn main() { println!("Rust Lucene scaffolding ready!"); }`. We'll use `src/lib/lib.rs` for library style, since it's a port. We'll also have a `cli/` subdirectory for CLI helpers if needed, but Makefile handles `build`, `run`, `test`, `references`, `help`.

## Decisions

1. Root `Cargo.toml`: single-crate `cargo new` style with `[workspace] = ["crates/*"]` placeholder in `[workspace]` section. Initially no sub-crates, so workspace members list empty or placeholder only.
2. `src/` structure: `src/lib/lib.rs` with simple placeholder `pub fn hello() { println!("Port of Lucene core"); }`. `src/cli/` optional — but for scaffolding, Makefile provides CLI targets (`cargo run` for CLI binary). Actually, `cargo build` builds the library; `cargo run` builds and runs default binary if `src/main.rs` exists. Simpler: `src/main.rs` with `fn main() { println!("Rust Lucene scaffolding ready!"); }` — this works with `cargo run` and `cargo build`. We'll use `src/main.rs` for simplicity at scaffolding stage.
3. Makefile defaults to `help`, lists targets: `help`, `references`, `build`, `test`, `run`, `clean`.
4. `.gitignore` contains `references/` pattern.
5. `make references` re-clones on every run (simplest, safe with `--depth 1`). Alternatively, update existing references; re-cloning is fine for reference material (small with --depth 1).
6. `make` help message: "usage: make [help|references|build|test|run|clean]" with brief description of each.

## Sources

- Cargo guide: https://doc.rust-lang.org/cargo/reference/manifest.html
- Makefile basics: https://www.gnu.org/software/make/
- Git clone: https://git-scm.com/docs/git-clone
- Rust workspace: https://doc.rust-lang.org/cargo/reference/manifest.html#the-workspace-manifest
