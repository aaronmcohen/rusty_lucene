# Changelog

 All notable changes to this project will be documented in this file.

 The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
 and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

 ## [Unreleased]

 ### Added
 - Initial project structure and basic search functionality
 - In-memory index implementation
 - Basic query DSL

 ### Changed
 - N/A

 ### Deprecated
 - N/A

 ### Removed
 - N/A

 ### Fixed
 - N/A

 ### Security
 - N/A

 ## [0.1.0] - 2026-11-21

 ### Added
 - Initial release of rusty_lucene library
 - Full-text search capabilities
 - Document indexing and querying
 - Memory and disk-backed index options
 - Basic highlighting functionality

 ### Changed
 - Initial API design finalized

 ### Deprecated
 - N/A

 ### Removed
 - N/A

 ### Fixed
 - N/A

 ### Security
 - N/A

 ## [0.0.1] - 2026-11-01

 ### Added
 - Project initialization
 - Basic Cargo.toml setup
 - Initial README and LICENSE

 ## [0.0.0] - 2026-10-01

 ### Added
 - Project conception and planning

 <!--
   Compare links - update these as versions are released
   Example: [0.1.0]: https://github.com/your-org/rusty_lucene/compare/v0.0.1...v0.1.0
   Example: [0.0.1]: https://github.com/your-org/rusty_lucene/compare/v0.0.0...v0.0.1
 -->

 ## Related

 - [Cargo.toml](Cargo.toml) - Project configuration
 - [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute
 - [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) - Community standards
 - [SECURITY.md](SECURITY.md) - Security policy

 ## Release Coordination

 Documentation updates are coordinated with code releases through the following process:

 1. **Release Branch Creation**: When a new version is ready, a release branch is created
 2. **Documentation Freeze**: Non-essential documentation changes are held until after the release
 3. **Version Updates**: The version number is updated in:
    - Cargo.toml
    - README.md badges
    - CHANGELOG.md unreleased section
 4. **Final Review**: Documentation is reviewed for accuracy and completeness
 5. **Simultaneous Release**: The release tag is created along with updated documentation
 6. **Post-Release**: Documentation is updated with any post-release fixes

 This ensures that users always have accurate documentation matching their installed version.
