# Research: Documentation Files

**Feature**: Documentation Files (specs/001-doc-files)
**Date**: 2026-11-21

## Unknowns from Technical Context

No NEEDS CLARIFICATION markers were found in the specification. All assumptions were documented and reasonable defaults were chosen based on:

1. **Language/Version**: Rust 1.75 - Based on project name "rusty_lucene" and industry standard for Rust projects
2. **Primary Dependencies**: Standard Rust ecosystem - The project is a search library, so it will use Rust standard library and potentially indexing crates
3. **Testing**: cargo test - Rust's native testing framework is the standard
4. **Project Type**: Library - The name suggests a reusable search library component

## Best Practices Researched

### README.md Best Practices
- Clear one-sentence project description at the top
- Installation instructions with copy-paste examples
- Basic usage examples with expected output
- Build/test status badges
- Link to documentation and contribution guidelines
- Reference to license

### LICENSE Selection
- MIT License: Permissive, allows commercial use, modification, distribution, private use
- Apache 2.0: Similar to MIT but with explicit patent grant and trademark use terms
- BSD 3-Clause: Permissive with clause requiring preservation of copyright notice

**Decision**: MIT License selected as default for open-source Rust libraries due to simplicity and permissiveness.

### CONTRIBUTING.md Best Practices
- Development environment setup (rustup, cargo)
- Code formatting (rustfmt)
- Linting (clippy)
- Testing procedures (cargo test)
- Pull request process
- Code review expectations
- Issue template references

### CODE_OF_CONDUCT.md Best Practices
- Contributor Covenant v2.1: Industry standard for open-source projects
- Clear scope of conduct
- Enforcement process with designated contact
- Reporting mechanism
- Sanctions for violations

### SECURITY.md Best Practices
- Supported versions list
- Security issue reporting process (preferably security.txt or dedicated email)
- Expected response time (typically 48-72 hours for acknowledged issues)
- Security update process description
- CVE request process reference

### CHANGELOG.md Best Practices
- Semantic Versioning 2.0.0 format
- Unreleased section for current development
- Versioned sections: Major.Minor.Patch
- Group changes: Added, Changed, Deprecated, Removed, Fixed, Security
- Links to relevant issues/PRs
- Date in YYYY-MM-DD format

## Decision Log

| Decision | Rationale | Alternatives Considered |
|----------|-----------|------------------------|
| MIT License | Simple, permissive, industry standard for Rust libraries | Apache 2.0 (more verbose, patent-focused), GPL (copyleft, may limit adoption) |
| Contributor Covenant v2.1 | Most widely adopted code of conduct for open-source | Custom code of conduct (requires legal review), no code of conduct (not recommended) |
| Security email (security@rusty-lucene.org) | Standard practice for security reporting | security.txt file (requires domain ownership), bug bounty program (out of scope for initial release) |
| Unreleased section in CHANGELOG | Standard practice to track upcoming changes | No unreleased section (harder to track what's coming) |

## Sources

- Keep a Changelog: https://keepachangelog.com/en/1.0.0/
- Semantic Versioning: https://semver.org/
- Contributor Covenant: https://www.contributor-covenant.org/
- GitHub Security Advisories: https://docs.github.com/en/code-security/security-advisories
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/