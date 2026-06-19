# Rusty Lucene Constitution

## Core Principles

### Compatibility & JNI Exposure
Every feature must expose a stable API via Java Native Interface (JNI) to ensure 100% compatibility with existing Apache Lucene tests without alteration.

### Minimal Dependency Baseline
Project dependencies are limited strictly to essential libraries only. No unnecessary transitive dependencies shall be introduced.

### Security By Design
Security considerations (e.g., validation, input sanitization, privilege separation) are mandated at the design phase for all new functionality.

### Descriptive Git Commit Discipline
Each discrete task must result in a single, clearly worded commit. Commits should reference JIRA tickets or GitHub Issues for traceability.

## Additional Constraints (Security & Performance)
- All cryptographic operations must use formally audited crates.
- Memory allocations exceeding 10MB per call shall be reviewed and optimized.

## Development Workflow & Review Process
- Pull Requests must undergo a two‑review minimum with at least one reviewer from the "architects" team.
- Automated CI runs linting, type checking, unit tests, and integration tests against Apache Lucene test suites.

## Governance
Amendments to this Constitution require a formal proposal, community vote via GitHub Issue discussion (.>24h), and a championed merge by a designated Constitution Owner.

**Version**: 1.0.0 | **Ratified**: 2025-06-18 | **Last Amended**: 2025-08-15