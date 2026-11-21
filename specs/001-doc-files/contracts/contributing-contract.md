# Contract: CONTRIBUTING.md Specification


**Feature**: Documentation Files
**Contract File**: `contracts/contributing-contract.md`
**Last Updated**: 2026-11-21


## Purpose


Define the exact requirements for the CONTRIBUTING.md file. This contract ensures that potential contributors have clear, actionable instructions for contributing to the project.


## Contract Terms


### 1. File Identity

- **File Name**: `CONTRIBUTING.md`
- **Location**: Project root (`/`)
- **Format**: Markdown (GitHub Flavored Markdown)
- **Encoding**: UTF-8


### 2. Required Sections (in recommended order)


#### 2.1 Introduction
- **Required**: Yes
- **Content**:
  - Welcome message for contributors
  - Brief project overview
  - What contributions are most needed
- **Length**: 2-4 paragraphs maximum


#### 2.2 Development Setup
- **Required**: Yes
- **Content**:
  - Prerequisites (rustup, cargo, etc.)
  - Clone instructions
  - Initial build/test commands
  - Verification steps
- **Format**: Numbered steps with code blocks


#### 2.3 Code Standards
- **Required**: Yes
- **Content**:
  - Code formatting (rustfmt)
  - Linting (clippy)
  - Testing requirements
  - Documentation requirements
- **Specifics**:
  - Command to run formatter: `cargo fmt`
  - Command to run linter: `cargo clippy`
  - Test command: `cargo test`
  - Documentation: `cargo doc`


#### 2.4 Pull Request Process
- **Required**: Yes
- **Content**:
  - Branch naming convention
  - Commit message format
  - PR description template
  - Review process
  - Merge criteria
- **Branch Naming**: `type/short-description` (e.g., `feature/add-search-api`)


#### 2.5 Community Guidelines
- **Required**: Optional but recommended
- **Content**:
  - Link to CODE_OF_CONDUCT.md
  - Expected response time for PR review
  - Working hours for maintainers (if applicable)


#### 2.6 Testing Requirements
- **Required**: Yes
- **Content**:
  - How to run tests
  - Test coverage requirements (if any)
  - Integration test process
  - Performance test process (if applicable)


### 3. Content Requirements


#### 3.1 Language and Tone
- **Primary Language**: English
- **Tone**: Welcoming, encouraging, clear
- **Technical Level**: Beginner-friendly but accurate


#### 3.2 Code Examples
- **Format**: GitHub Flavored Markdown code blocks
- **Syntax Highlighting**: `bash` or `shell` for shell commands
- **Completeness**: All commands must be copy-paste runnable
- **Comments**: Explain why each step is necessary


#### 3.3 Links
- **Internal Links**: Relative paths to other docs
- **External Links**: Full URLs with descriptive anchor text
- **Broken Links**: Zero tolerance


### 4. Quality Gates


#### 4.1 Mandatory Checks
- [ ] All required sections present
- [ ] Setup instructions are copy-paste runnable
- [ ] All commands have been tested
- [ ] rustfmt and clippy commands specified
- [ ] Test command is documented
- [ ] Branch naming convention is clear
- [ ] Commit message format is specified
- [ ] PR review process is documented
- [ ] No TODO or FIXME comments in content


#### 4.2 Recommended Checks
- [ ] Flesch-Kincaid Grade Level 8-10
- [ ] Consistent formatting throughout
- [ ] Active voice used predominantly
- [ ] Gender-neutral language
- [ ] No implementation details beyond what's needed for contribution


### 5. Change Management


#### 5.1 When to Update
- When development process changes
- When tooling changes (rustup, cargo, etc.)
- When PR review process changes
- When community guidelines change
- When project scope changes significantly


#### 5.2 Update Process
1. Update contract version
2. Update CONTRIBUTING.md content
3. Test all setup instructions from a clean state
4. Verify all commands work as documented
5. Update CHANGELOG.md with CONTRIBUTING.md update note


### 6. Validation Methods


#### 6.1 Automated Validation
```bash
# Test setup instructions
# (Manual verification recommended)

# Check for broken links
markdown-link-check CONTRIBUTING.md

# Verify code blocks are valid
# (Manual review recommended)
```


#### 6.2 Manual Validation
1. Read CONTRIBUTING.md from perspective of new contributor
2. Follow all setup instructions step by step
3. Attempt to make a small test contribution
4. Verify PR process description matches actual process
5. Check all links and commands


### 7. References


- [GitHub Contributing Guide](https://docs.github.com/en/community/contributing)
- [Open Source Guides: How to Contribute](https://opensource.guide/how-to-contribute/)
- [Cargo Book: Contributing](https://doc.rust-lang.org/book/ch11-03-cargo-commands.html)
- Project Constitution: `.specify/memory/constitution.md`


### 8. Contract Validation


This contract is validated by:
- Manual review against this specification
- Testing of all setup instructions from clean state
- Link validation
- Code example validation


## Revision History


| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-11-21 | speckit-agent | Initial contract definition |