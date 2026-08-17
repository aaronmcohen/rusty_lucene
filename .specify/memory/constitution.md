# Rusty Lucene Constitution

## Core Principles

### Library-First
Every feature starts as a standalone Rust library; libraries must be self-contained, independently testable, and well-documented with a clear purpose—no organizational‑only libraries are allowed.

### CLI Interface
All libraries expose functionality via a command‑line interface. Input/output uses standard text protocols: `stdin`/arguments → `stdout`, errors → `stderr`. Both human‑readable and JSON formats are supported.

### Test‑First (NON‑NEGOTIABLE)
Test‑Driven Development is mandatory: tests are written, approved, then make the tests fail; implement the feature; finally refactor while keeping tests passing.

### JNI‑Compatibility Testing
Ported Rust code must be verified by running the corresponding `Test*.java` files against the JNI interface, ensuring 100% compatibility with Apache Lucene semantics.

### 100% Test Coverage
All ported Rust implementations must achieve 100% code coverage as measured by the `cargo test` suite.

## Project Orchestration
All build, test, deploy, and release processes are orchestrated via a `Makefile` in the repository root. No external scripts or CI‑specific files are permitted.

## Governance

- Constitution supersedes all other project practices.
- Amendments require documentation, community approval, and a migration plan.
- Versioning follows semantic rules: MAJOR for backward‑incompatible changes, MINOR for new principles, PATCH for clarifications.

**Version**: 1.0.0 | **Ratified**: 2025-08-17 | **Last Amended**: 2025-08-17
