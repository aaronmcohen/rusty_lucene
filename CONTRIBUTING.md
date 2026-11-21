# Contributing to rusty_lucene

 Welcome to rusty_lucene! We're excited to have you help us build this Rust search library. This document outlines how to contribute to the project.

 ## Development Setup

 ### Prerequisites

 - **Rustup**: Install from https://rustup.rs/
 - **Cargo**: Rust's package manager (installed with Rustup)

 ### Cloning the Repository

 ```bash
 git clone https://github.com/your-org/rusty_lucene.git
 cd rusty_lucene
 ```

 ### Initial Build Verification

  Verify the project builds:

 ```bash
 cargo build
 ```

  Run tests:

 ```bash
 cargo test
 ```

 ## Code Standards

 ### Formatting

 We use `rustfmt` for code formatting. Format your code before committing:

 ```bash
 cargo fmt
 ```

 ### Linting

 We use `clippy` for linting. Check your code:

 ```bash
 cargo clippy
 ```

 ### Testing Requirements

 - All new functionality must include tests
 - Aim for high test coverage
 - Tests should be clear and demonstrate expected behavior

 ## Pull Request Process

 1. Fork the repository
 2. Create a feature branch: `git checkout -b feature/my-new-feature`
 3. Make your changes with clear commit messages
 4. Ensure all tests pass: `cargo test`
 5. Ensure code is formatted: `cargo fmt`
 6. Ensure no linting errors: `cargo clippy`
 7. Push to your fork and submit a pull request

 ### Commit Message Format

 We follow conventional commits:

 - `feat:` New feature
 - `fix:` Bug fix
 - `docs:` Documentation changes
 - `style:` Code style changes (formatting, etc.)
 - `refactor:` Code refactoring
 - `test:` Adding tests
 - `chore:` Maintenance tasks

 Example: `feat: add phrase query support`

 ## Code Review

  - All PRs require review from at least one maintainer
  - Reviewers will check for correctness, clarity, and adherence to project standards
  - Address review comments promptly
  - CI checks must pass before merge

 ## Community Guidelines

 This project follows the Contributor Covenant Code of Conduct. Please be welcoming, inclusive, respectful, and constructive in all interactions.

 See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for detailed guidelines.

 ## Maintainer Process

 New maintainers are added through the following process:

 1. **Contribution Track Record**: Contributors must have made consistent, high-quality contributions over at least 3 months
 2. **Community Trust**: Demonstrate understanding of project values and community standards
 3. **Formal Nomination**: Existing maintainers nominate candidates during a maintainers' meeting
 4. **Voting**: All current maintainers vote on the nomination (requires 2/3 majority)
 5. **Onboarding**: New maintainers receive:
    - Access to project administration tools
    - Introduction to the maintainers' communication channel
    - Documentation on maintainers' responsibilities and processes
    - A mentor (existing maintainer) for the first 3 months

 **Maintainer Responsibilities**:
 - Review and merge pull requests
 - Respond to issues and security reports
 - Participate in release planning
 - Help onboard new contributors
 - Uphold the Code of Conduct

 If you're interested in becoming a maintainer, start by contributing regularly and engaging with the community.
