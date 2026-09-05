# Quickstart: Rust Lucene Scaffolding

## What this gives you

A minimal Rust Cargo project scaffold with a Makefile that provides:

- `make` (or `make help`) – shows usage
- `make references` – clones Apache Lucene (core) to `references/` (git-ignored)
- `cargo build`
- `cargo test`
- `cargo run`

The root `Cargo.toml` is a single-crate file with a `[workspace]` placeholder for future sub-crates.

## First steps

```bash
# 1. Clone or navigate to the repository root
cd /path/to/rust-lucene

# 2. Run help (default target)
make                    # or: make help

# 3. Build the project
cargo build

# 4. Run the default binary
cargo run

# 5. Clone Lucene reference material
make references

# 6. Verify references is git-ignored
git status              # should show references/ as untracked
git diff --cached .gitignore  # should be empty (references/ not staged)

# 7. Run tests (when applicable)
cargo test
```

## Makefile targets explained

| Target | Description |
|--------|-------------|
| `help` (default) | Shows this quickstart usage message |
| `references` | Clones https://github.com/apache/lucene.git into `references/` with `--depth 1` for speed |
| `build` | Runs `cargo build` |
| `test` | Runs `cargo test` |
| `run` | Runs `cargo run` (default binary) |
| `clean` | Removes `references/` (reference material) |

> Note: `references/` is git-ignored, so it won't be accidentally committed.

## Expected behavior

After running `make references`:

- `references/` directory exists with Lucene source
- `git status` shows `references/` as untracked (not staged)
- `git add references` would stage it, but `git commit` would not include it unless explicitly added

After running `cargo build`:

- Build succeeds without requiring any manual file creation
- No build errors

## Troubleshooting

- **No internet?** `make references` will fail with a clear git error. The spec requires graceful failure with a helpful message.
- **`references/` already exists?** `make references` re-clones with `--depth 1` (fast and clean).
- **Running `make` with no args doesn't show help?** Check that `.PHONY: all help` is in the Makefile and `all: help` is the default.

## Repository layout (at root)

```
rust-lucene/
├── Cargo.toml
├── src/
│   └── main.rs          # "Rust Lucene scaffolding ready!" message
├── Makefile
├── .gitignore           # ignores references/
├── references/          # ← git-ignored; contains Lucene core
└── specs/001-rust-lucene-scaffolding/  # documentation only
```

## Next steps (post-scaffolding)

- Create sub-crates under `crates/` directory using workspace members
- Replace `src/main.rs` placeholder with actual Lucene core interface definitions
- Add real dependencies in `Cargo.toml` [dependencies] section
- Implement `make` targets for port-specific build steps (e.g., `port`, `benchmark`)
- Add unit tests in `tests/unit/`, integration tests in `tests/integration/`

