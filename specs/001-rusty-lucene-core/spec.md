# Feature Specification: rusty‑lucene‑core

## Summary
Create the initial, minimal Rust crate **rusty-lucene-core** at the repository root. This crate will serve as a placeholder for future porting of Apache Lucene’s `core` module and will contain no Lucene source code or functionality. All other Lucene modules remain out‑of‑scope for this specification.

## Background & Context
- The repository holds a reference copy of Apache Lucene under `@references/lucene`.  
- The **first** Java project to be ported is `lucene/core`.  
- Subsequent ports (e.g., `analysis`, `codecs`) will be handled in later specs.  
- This spec only establishes the crate scaffold; it does **not** perform any code translation, wrapping, or publishing.

## Functional Requirements
| # | Requirement | Success Criterion |
|---|-------------|-------------------|
| FR‑1 | Provide a minimal Rust crate named `rusty-lucene-core` at the repository root. | Directory `rusty-lucene-core/` exists with a valid `Cargo.toml`. Running `cargo check` succeeds without errors. |
| FR‑2 | Crate metadata must include name, version (`0.1.0`), authors, edition (`2021`). | `cargo metadata --format-version 1` lists these fields exactly as specified. |
| FR‑3 | Include a placeholder **README.md** describing the purpose of the crate and noting that actual Lucene code will be added later. | README contains at least three sentences: (a) brief description, (b) note about upcoming porting, (c) link to this specification. |
| FR‑4 | Add an empty `src/lib.rs` as the entry point for future implementations. | File exists and compiles (`#![allow(dead_code)]` permitted). |
| FR‑5 | tests/placeholder_test.rs |
| FR‑6 | Adhere to the project’s **Constitution principles** (Minimal Dependency Baseline, Descriptive Commit Discipline, etc.) as documented in `.specify/memory/constitution.md`. | No runtime dependencies beyond the Rust standard library; commit messages follow constitution guidelines.

## Non‑Functional Requirements
- **Zero Lucene source files** may reside in `rusty-lucene-core/` until this spec is marked *completed*.  
- Build must finish on a stable Rust toolchain within **5 seconds** on a typical development machine.  
- No external crates (aside from optional dev dependencies for testing) are allowed.

## Success Criteria (Business‑Focused)
1. **Stakeholder Acceptance:** Product owners can confirm the existence of an empty, ready‑to‑receive crate without any Lucene code.  
2. **Technical Readiness:** CI pipeline runs `cargo check` and `cargo test` on `rusty-lucene-core` and reports **green** status.  
3. **Documentation Completeness:** The placeholder README is reviewed by at least one non‑technical stakeholder and deemed understandable.

All criteria are measurable, technology‑agnostic, and verifiable without knowledge of future Rust implementation details.

## Assumptions
- Write permission exists for the repository root in the current environment.  
- A stable Rust toolchain (`rustup default stable`) is installed.  
- No pre‑existing directory named `rusty-lucene-core` exists; if it does, this spec will safely overwrite its contents.

## Out‑of‑Scope / Exclusions
| Item | Reason |
|------|--------|
| Porting any Java class from `lucene/core` to Rust code | Deferred to a later dedicated specification. |
| Publishing the crate to crates.io or any registry | Publication is outside this scaffolding effort. |
| Adding feature flags, configuration options, CLI entry points | Not required for an empty placeholder. |
| Writing documentation beyond the placeholder README | Additional docs will be added in subsequent features. |

## Acceptance Test Plan (High‑Level)
1. **Clone** the repository and verify `rusty-lucene-core/` does not exist initially.  
2. **Execute** the scripted steps defined below (or via CI).  
3. **Verify**:   
   - Directory structure matches: `rusty-lucene-core/Cargo.toml`, `src/lib.rs`, `tests/placeholder.rs`, `README.md`.   
   - `cargo check` and `cargo test` both succeed.   
   - No files from `@references/lucene/lucene/core` appear inside the crate directory.

Passing all three verification points marks the specification as **accepted**.

## Governance Alignment
- **Minimal Dependency Baseline:** Only the Rust standard library is declared → satisfies principle 1 of the Constitution.  
- **Descriptive Commit Discipline:** Future commits related to this feature will follow commit‑message conventions from `.specify/memory/constitution.md`.  
- **Version Control & Branch Management:** A dedicated Git branch (e.g., `feature/rusty-lucene-core`) should be created for any modifications; the specification itself is version‑controlled via its markdown file.

## Clarifications
**Out‑of‑scope items (as defined in the spec):**
- Porting any Java class from `lucene/core` to Rust code  
- Publishing the crate to crates.io or any registry  
- Adding feature flags, configuration options, CLI entry points  
- Writing documentation beyond the placeholder README
