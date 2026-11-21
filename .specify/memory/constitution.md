<!--
SYNC IMPACT REPORT
Version change: 1.0.0 → 1.1.0 (MINOR: Added new principles for Rusty Lucene project structure, licensing, and documentation requirements)
Modified principles: 
  - I. Library-First: Added Rust best practices and Lucene compatibility requirement
  - II. Documentation-First: Added Apache 2.0 license requirement and required documentation files
  - IV. Observability: Added GitHub Pages documentation generation requirement
Added sections:
  - I. Lucene Compatibility & Rust Best Practices (new principle replacing generic library-first)
  - Mono-repo Structure Requirements (new section for project organization)
  - License & Compliance (new section for Apache 2.0 requirements)
Removed sections: None
Follow-up TODOs: None
-->

# rusty_lucene Constitution

## Core Principles

### I. Lucene Compatibility & Rust Best Practices
The Rust port must adopt Rust best practices while maintaining 100% compatibility with the Lucene interfaces. Rust crates use the Rusty Lucene naming convention based on original Lucene package component functionality. All public APIs must match Lucene signatures and behavior to ensure seamless migration and compatibility.

### II. Documentation-First
Every feature must include complete documentation before merge; Documentation is tested alongside code; No undocumented public interfaces. The project root must contain: README.md, LICENSE, CODE_OF_CONDUCT.md, CONTRIBUTING.md, SECURITY.md, and CHANGELOG.md. All documentation files must pass link checking and spell checking as part of CI.

### III. Test-First (NON-NEGOTIABLE)
TDD mandatory: Tests written → User approved → Tests fail → Then implement; Red-Green-Refactor cycle strictly enforced. The Java compatibility layer validity is validated by copying original Lucene unit tests to the third set of sub-projects and running them without modification.

### IV. Observability
Text I/O ensures debuggability; Structured logging required; Documentation must be clear and debuggable. The GitHub Pages site must be generated with all documentation.

### V. Versioning & Breaking Changes
MAJOR.MINOR.PATCH format; Breaking changes require migration plan; Start simple, YAGNI principles. Each sub-project follows semantic versioning. Cross-sub-project version alignment must be explicitly declared when dependencies are updated. Release notes MUST document all breaking changes.

## Mono-Repo Structure Requirements

The project is a mono-repo of three groups of sub-projects:

1. **Rust Crates**: Rust crates using the Rusty Lucene name convention based on the original Lucene package component functionality. The Rust port should adopt Rust best practices while maintaining 100% compatibility with the Lucene interfaces.

2. **Java JNI Packages**: Java JNI packages using the Rusty Lucene name convention which provides a Java interface to the Rust libraries.

3. **Java Compatibility Packages**: Java packages using the original Lucene naming convention to provide 100% compatibility with the original Lucene code using the methods from the Rusty Lucene Java JNI packages. Compatibility is validated by copying the original unit tests to the third set of sub-projects and running them without modification.

The `education/` folder must exist in the project root, containing documentation that explains the source code as if it were an introduction to computer science class. The `references/` directory must contain git sub-modules representing the code being ported from.

## License & Compliance

All code must be licensed under the Apache License 2.0 to align with the Apache Lucene project. This license must be included in the project root as LICENSE.

## Development Workflow

The workflow should be minimal as there is just one developer on the project. While GitHub Actions is used for CI/CD, all top-level orchestration should be done via a Makefile. None of the top-level orchestration actions (build, test, etc.) should require GitHub Actions and is done via the top-level Makefile.

All PRs must verify compliance with constitution; Complexity must be justified; Use guidance files for runtime development.

## Governance

All PRs/reviews must verify compliance; Complexity must be justified; Use guidance files for runtime development guidance.

**Version**: 1.1.0 | **Ratified**: 2026-11-21 | **Last Amended**: 2026-11-21