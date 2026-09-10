<!-- SYNC_IMPACT_REPORT:
Version: 1.1.0 → 1.2.0 (MINOR bump)
Changes:
- Added new NON-NEGOTIABLE principle "VI. Clean Code Discipline": Functions limited to max 20 lines, do one thing without unwanted side effects, error/edge cases checked at the very beginning and return early to keep code flat, never hardcode raw values (e.g., numeric constants like `1.0`), use named constants or configuration variables that clearly communicate domain meaning (e.g., `DEFAULT_INDEX_SHARD_COUNT` for shard count defaults). Use comments only for non-obvious reasoning, business logic constraints, or historical context that cannot be inferred from code alone. Avoid comments that merely restate what the code already expresses (e.g., "x is a counter" when variable name is `counter`). Naming clearly explains what a function/variable does (e.g., `document_parser` over `data`). This principle applies to all production code.
- Updated "V. Code Quality": changed label from `(JNI + Coverage)` to `(JNI + Coverage + Clean Code)` to reflect inclusion of clean code principles alongside existing standards.
- Updated "Additional Constraints": added Clean Code Discipline guideline (20-line function guideline with practical exception for small utilities, no magic constants).
- Updated "Development Workflow": added clean code review checklist (check function length ≤20 lines, naming clarity, look for nested if statements to restructure).
- Governance note updated: clean code discipline is non-negotiable for production code; exceptions for prototyping must be documented and reviewed.
Rationale: Clean code discipline improves readability, maintainability, and reduces bugs while complementing existing Documentation-as-Teaching and Test-First principles.
Last amended: 2025-08-15
-->

# Rusty Lucene Constitution

## Core Principles

### I. Documentation-as-Teaching (NON-NEGOTIABLE)
Every Rust source code file (`*.rs`), module (`mod`), and crate must include documentation written as if teaching an introduction to computer science — using plain language, avoiding jargon, providing concrete analogies and small examples, answering "why" and "how" for readers unfamiliar with advanced concepts. A peer review (anyone, not necessarily an expert) must approve the documentation before the code can be merged.

### II. Library-First
Every feature starts as a standalone Rust library; libraries must be self-contained and independently testable. No organizational-only libraries are allowed.

### III. CLI Interface
All libraries expose functionality via a command-line interface. Input/output uses standard text protocols: `stdin`/arguments → `stdout`, errors → `stderr`. Both human-readable and JSON formats are supported.

### IV. Test-First (NON-NEGOTIABLE)
Test-Driven Development is mandatory: tests are written and approved first, then the feature is implemented causing tests to fail, then the implementation works, and finally refactoring happens while keeping tests passing. Red-Green-Refactor cycle strictly enforced.

### V. Code Quality (JNI + Coverage + Clean Code)
All Rust implementations (including ported code) must achieve 100% code coverage as measured by `cargo test` and follow JNI-Compatibility Testing standards — ported Rust code must be verified by running the corresponding `Test*.java` files against the JNI interface, ensuring 100% compatibility with Apache Lucene semantics. Additionally, code must adhere to clean code principles.

### VI. Clean Code Discipline (NON-NEGOTIABLE)
Functions should be limited to a maximum of 20 lines and do one thing — perform a single, focused task without side effects to global state or external mutable state when possible. Error and edge case checking should occur at the very beginning of the function, returning early when appropriate to keep code flat and readable. Never hardcode raw values (e.g., numeric constants like `1.0`, `0.5`); instead, extract them into named constants or configuration variables that clearly communicate domain meaning (e.g., `DEFAULT_INDEX_SHARD_COUNT` for shard count defaults). Use comments only to explain non-obvious reasoning, business logic constraints, or historical context that cannot be inferred from code alone. Avoid comments that merely restate what the code already expresses (e.g., "x is a counter" when the variable name is `counter`). Naming should clearly explain what a function/variable does — avoid vague names like `data` or `item`; use descriptive names like `document_parser` or `search_result`. This principle holds as non-negotiable for all production code.

## Additional Constraints
Documentation-as-Teaching style guide: Use plain language (no acronyms without explanation), avoid domain-specific jargon; use analogies familiar to beginners; provide small runnable examples; answer the "why" behind the API design; keep documentation concise but complete. All libraries must have module-level documentation for the crate root and module headers.

Clean Code Discipline: Follow the 20-line function guideline as a practical guideline (exceptions are allowed for small utility functions where maintaining readability would be worse than exceeding 20 lines); never hardcode numeric or magic constants (e.g., `1.0`, `0.5`) — use named constants or configuration via env vars/explicit params for tunable values. Tests must also follow Clean Code Discipline — test functions should be short and focused, with clear assert messages that explain the expected behavior rather than just repeating test names.

## Development Workflow
All build, test, deploy, and release processes are orchestrated via a `Makefile` in the repository root. No external scripts or CI-specific files are permitted. Follow the Test-First discipline: write tests → make them fail → implement → refactor. Use `make` targets for build, test, lint, and release.

Review PRs for clean code adherence: check function length (aim ≤20 lines), naming clarity, and appropriate use of constants. Look for deeply nested if statements; restructure to return early when possible.

## Governance
Constitution supersedes all other project practices. Amendments require documentation, community approval (at least one other maintainer), and a migration plan if backward compatibility is affected. Complexity increases maintenance cost — justify new complexity before adding it. Clean code discipline is non-negotiable for production code; exceptions for prototyping must be documented and reviewed. Versioning follows semantic rules: MAJOR for backward-incompatible changes, MINOR for new principles or non-breaking additions, PATCH for clarifications, wording fixes, and typo corrections. Governance review is required for all new principles.

**Version**: 1.2.0 | **Ratified**: 2025-08-17 | **Last Amended**: 2025-08-15
