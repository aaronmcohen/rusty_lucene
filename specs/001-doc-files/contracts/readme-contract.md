# Contract: README.md Specification

**Feature**: Documentation Files
**Contract File**: `contracts/readme-contract.md`
**Last Updated**: 2026-11-21

## Purpose

Define the exact structure and content requirements for the README.md file. This contract serves as the source of truth for what the README must contain and how it must be structured.

## Contract Terms

### 1. File Identity
- **File Name**: `README.md`
- **Location**: Project root (`/`)
- **Format**: Markdown (GitHub Flavored Markdown)
- **Encoding**: UTF-8

### 2. Required Sections (in order)

#### 2.1 Project Header
- **Required**: Yes
- **Content**: 
  - Project name as H1 heading (`# Project Name`)
  - Short description (1-2 sentences)
  - Optional: Project logo or badge area

#### 2.2 Badges
- **Required**: Yes
- **Content**: At least one badge showing:
  - Build status (if CI configured)
  - Current version
  - License type
  - Language (Rust)
- **Format**: Inline badges using shields.io or similar service
- **Position**: Directly below project header

#### 2.3 Description
- **Required**: Yes
- **Content**: Clear, concise description of what the project does
- **Length**: 1-3 sentences maximum
- **Tone**: Technical but accessible

#### 2.4 Quick Start / Installation
- **Required**: Yes
- **Content**:
  - Prerequisites (if any)
  - Installation command(s)
  - Verification step
- **Format**: Code block with copy-paste ready commands

#### 2.5 Basic Usage
- **Required**: Yes
- **Content**: 
  - Simple example showing core functionality
  - Expected output
  - Explanation of what the code does
- **Format**: Code block with Rust syntax highlighting

#### 2.6 Features
- **Required**: Optional but recommended
- **Content**: Bulleted list of key features
- **Length**: 3-10 items maximum

#### 2.7 Documentation Links
- **Required**: Yes
- **Content**: Links to:
  - Full documentation (if separate)
  - API reference
  - Examples
  - Contributing guide
- **Format**: Bulleted list with descriptive link text

#### 2.8 License
- **Required**: Yes
- **Content**: License badge and link to LICENSE file
- **Position**: At the bottom of the file

### 3. Content Requirements

#### 3.1 Language and Tone
- **Primary Language**: English
- **Tone**: Professional, welcoming, concise
- **Technical Level**: Intermediate (assumes basic Rust knowledge)

#### 3.2 Code Examples
- **Format**: GitHub Flavored Markdown code blocks
- **Syntax Highlighting**: `rust` for Rust code
- **Completeness**: Must be copy-paste runnable (when possible)
- **Comments**: Explain the "why" not the "what"

#### 3.3 Links
- **Internal Links**: Relative paths to other docs
- **External Links**: Full URLs with descriptive anchor text
- **Broken Links**: Zero tolerance - all links must be valid

### 4. Quality Gates

#### 4.1 Mandatory Checks
- [ ] All required sections present
- [ ] Code examples are syntactically valid
- [ ] All internal links are valid
- [ ] License badge links to LICENSE file
- [ ] Badges display correctly (test in markdown viewer)
- [ ] No TODO or FIXME comments in content

#### 4.2 Recommended Checks
- [ ] Flesch-Kincaid Grade Level 8-10
- [ ] Consistent heading hierarchy (H1 → H2 → H3)
- [ ] Active voice used predominantly
- [ ] Gender-neutral language
- [ ] No implementation details (frameworks, specific APIs)

### 5. Change Management

#### 5.1 When to Update
- When project name changes
- When installation process changes
- When core functionality changes significantly
- When project moves to a new repository
- When license changes

#### 5.2 Update Process
1. Update contract version
2. Update README content
3. Verify all quick start examples still work
4. Update CHANGELOG.md with README update note

### 6. Contract Validation

This contract is validated by:
- Manual review against this specification
- Automated link checking (markdown link checker)
- Code example validation (cargo check if examples are Rust code)
- Spell checking (optional)

## References

- [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
- [GitHub Docs: Markdown](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax)
- Project Constitution: `.specify/memory/constitution.md`

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-11-21 | speckit-agent | Initial contract definition |