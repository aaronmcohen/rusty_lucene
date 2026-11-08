<!-- Sync Impact Report:
- Version change: 1.0.4 → 1.0.5
- List of modified principles: Additional Constraints updated with crates/ directory requirement for Rust sub-projects
- Added sections: None
- Removed sections: None
- Follow-up TODOs: None
-->
# rusty_lucene Constitution

## Core Principles

### I. Meta Project Organization
This project is a meta project coordinating three sub-projects:
- **Rust sub-projects**: Contain Rust implementation based on the Apache Lucene Project (https://github.com/apache/lucene).
- **Java JNI sub-projects**: Provide Java interfaces (JNI) to the Rust implementation.
- **Java projects**: Provide Apache Lucene library compatibility abstraction for the Java JNI sub-projects, ensuring existing Java code can use the native implementation without modification.
Each sub-project must be self-contained, independently buildable, and have a clear public interface.

### II. Sub-project Independence
The Rust, Java JNI, and Java libraries sub-projects operate with separate build systems, test suites, and release cycles. Cross-sub-project dependencies must be explicit, versioned, and documented. The Java JNI layer exposes Rust functionality via JNI; the Java compatibility abstraction layer ensures drop-in replacement for standard Apache Lucene classes.

### III. Minimal Governance
Governance is minimal due to single-developer maintenance. All decisions are made unilaterally by the sole developer. Documentation is updated to reflect decisions, but formal review or approval processes are not required.

### IV. Consistent Tooling
Shared tooling for CI/CD, code formatting, and documentation applies across all sub-projects to ensure a uniform development experience.

## Additional Constraints

All sub-projects must follow language-specific best practices. Specifically:
- **Rust sub-projects**: Must follow Rust API documentation best practices as outlined in https://rust-lang.github.io/api-guidelines/documentation.html.
- **Rust sub-projects**: Must be located in the `crates/` directory.
- **Java projects**: Must mirror the original Apache Lucene documentation style, structure, and content to ensure compatibility and familiarity for existing users.
- All code must be licensed under the Apache License 2.0 to align with the Apache Lucene project.
- The project root must contain: README.md, LICENSE, CODE_OF_CONDUCT.md, CONTRIBUTING.md, SECURITY.md, and CHANGELOG.md.
- An `education/` folder must exist in the project root, containing documentation that explains the source code as if it were an introduction to computer science class.
New sub-projects may only be added with justification recorded in the project documentation.

## Development Workflow

As a single-developer project, the workflow is streamlined: work happens directly on the relevant sub-project branch, with changes merged after minimal self-review. Feature flags may be used for experimental work affecting multiple sub-projects.

## Governance

This constitution supersedes any prior development practices. Amendments are made by updating this document directly; since there is only one developer, no external approval is required.

**Version**: 1.0.5 | **Ratified**: 2026-11-03 | **Last Amended**: 2026-11-03
