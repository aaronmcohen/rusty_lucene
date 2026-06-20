# Rusty Lucene Constitution

## Core Principles

### Compatibility & JNI Exposure
Every feature must expose a stable API via Java Native Interface (JNI) to ensure 100% compatibility with existing Apache Lucene tests without alteration.

### Minimal Dependency Baseline
Project dependencies are limited strictly to essential libraries only. No unnecessary transitive dependencies shall be introduced.

### Security By Design
Security considerations (e.g., validation, input sanitization, privilege separation) are mandated at the design phase for all new functionality.

### Descriptive Git Commit Discipline
Each commit must represent a single logical change with a clear subject line. Include references to associated tickets or issues and employ imperative mood (`Add X`, `Fix Y`). Rebase before merging; retain clean history.

## Additional Constraints (Security & Performance)
- All cryptographic operations must use formally audited crates.
- Memory allocations exceeding 10MB per call shall be reviewed and optimized.

## Development Workflow & Review Process
### Specification Authoring
**All new specs should be authored on their own branch.** Create a dedicated feature branch (e.g., `spec/issue-123`) for each specification, develop changes there, then open a Pull Request against `main`.

- **Git Flow**: Use feature branches, rebase onto main, then open a PR for each change.
- **CI Checks**: Automated linting, type checking, unit tests, and integration tests must pass before merge.
- **Self‑Review**: As the sole developer, perform thorough code review on every PR (review checklist includes compile success, test coverage, documentation updates).

## Governance (Solo Developer)
### Amendment Process
Because there is a single maintainer, any change to this constitution is approved by the author's own decision. Document the rationale in commit messages or an amendment log.

### Compliance Review
- **Pre‑merge**: Run all CI checks locally; ensure 100% test pass and no lint warnings.
- **Weekly Health Check**: Verify that commit history remains clean (no merge commits, consistent formatting).

### Enforcement
- Enforce the Git Commit Discipline strictly via pre‑commit hook or local alias.
- Treat any violation of CI failures as blocking – fix before pushing to main.
- Maintain a CHANGELOG entry for each amendment.

**Version**: 1.0.3 | **Ratified**: 2025-06-18 | **Last Amended**: 2026-06-20