# Feature Specification: jni-wrappers

**Feature Branch**: `002-jni-wrappers`

**Created**: [2026-04-22]

**Status**: Draft → Validated

**Input**: User description: "Similar to @crates/ which contains all the rust sub-crates there should be a @jni/ which contains all the java projects that are java Native Interface wrappers of each sub-crate from @crates/"

## Clarifications

### Session 2026-04-22
- Q: Should Success Criteria include time bounds (≤ 1 day for wrapper generation, ≤ 30 minutes for wrapper update)? → A: No — the user stated "success criteria '≤ 1 day ...' and '≤ 30 minutes ...' is not required."
- Q: Should `make build`, `make test`, and `make clean` in the top-level Makefile automatically handle JNI wrapper projects under `@jni/` in addition to the Rust workspace? → A: Yes — `make build` should build both the Rust workspace and JNI wrapper java projects; `make test` should test both Rust and JNI; `make clean` should clean both the Rust workspace and `@jni/` directory.

### Session 2026-09-13
- Q: Should JNI wrapper projects be named `@jni/<crate-name>` instead of `@jni/<crate-name>-jni`? → A: Yes (user input specifies `@jni/<crate-name>`)
- Q: Should we create a minimal `@jni/<crate-name>/` directory only for Rust sub-crates that export at least one JNI-suitable native function (skip for crates with no suitable exports)? → A: Yes (AM1 decision — only create wrapper for crates with JNI-suitable exports)
- Q: Should we implement the minimal `jni-update` script (minimal commands to regenerate only the affected JNI wrapper)? → A: Yes (AM2 decision)

### User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.

  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Create JNI wrapper for a Rust crate (Priority: P1)
A Java developer needs to get a Java Native Interface (JNI) wrapper project for each Rust sub-crate under `@crates/`. The system should automatically discover Rust sub-crates and generate the corresponding Java JNI wrapper project under `@jni/`.

**Why this priority**: Enables Java teams to consume Rust functionality through standard JNI interfaces without manually writing wrapper boilerplate for each crate.

**Independent Test**: Given `@crates/` contains N Rust sub-crates, when the JNI wrapper generation tool runs, it creates a Java JNI wrapper project under `@jni/<crate-name>/` only for crates that export at least one JNI-suitable native function (crates with no JNI-suitable exports are skipped).

**Acceptance Scenarios**:
1. Given there are N Rust sub-crates in `@crates/`, when the JNI wrapper generation tool runs, then exactly the applicable crates (those with JNI-suitable native exports) have `@jni/<crate-name>/` wrapper projects created, each with proper project structure (no `-jni` suffix).
2. Given a Rust sub-crate `rusty-lucene-core` exports native functions suitable for JNI, When the Java `@jni/rusty-lucene-core/` wrapper project is generated, Then the `@jni/rusty-lucene-core` directory contains a minimal Java interface (e.g., `RustyLuceneJni`) that declares JNI-compatible methods matching the Rust native export signatures.

### User Story 2 - Build and test JNI wrapper integration (Priority: P2)
A developer needs to build both the Rust native library for a crate and its JNI wrapper, then verify the Java code can call into the Rust native library correctly.

**Why this priority**: Confirms the generated wrapper actually works at runtime, which is the core value of this feature.

**Independent Test**: Build the Rust native library for a crate and its JNI wrapper project, run a simple Java test that calls a JNI method, and observe the correct Rust function responds.

**Acceptance Scenarios**:
1. Given the Rust sub-crate `rusty-lucene-core` implements a native function `search`, When the Java `@jni/rusty-lucene-core/` wrapper is built and the Rust library is loaded via JNI, Then calling `RustyLuceneJni.search(query)` returns the expected search results from Rust.
2. Given a Rust sub-crate has dependencies, When the JNI wrapper for that crate is built, Then the wrapper's build configuration correctly links against all dependent Rust native libraries (no linker errors).

### User Story 3 - Maintain consistency when Rust crates change (Priority: P3)
When a Rust sub-crate is updated (e.g., adds a new native function, modifies an existing one, or removes a function), the corresponding Java JNI wrapper should be updated automatically or with minimal manual intervention.

**Why this priority**: Prevents Java wrappers from becoming stale or broken when Rust APIs evolve.

**Independent Test**: Modify a Rust sub-crate's native export (add, modify, or remove a function), then regenerate the JNI wrapper for that crate, and verify the Java wrapper correctly reflects the change.

**Acceptance Scenarios**:
1. Given a Rust sub-crate adds a new native function `rank`, When the JNI generation tool runs for that crate, Then the Java method `rank()` is created with the correct JNI signature (e.g., `long rank(long id)`).
2. Given a Rust sub-crate removes a native function that was previously exposed via JNI, When the JNI generation tool runs, Then the Java method for that removed function is removed from the wrapper (no dangling/unused JNI method).

### Edge Cases

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right edge cases.
  -->

- What happens when a Rust sub-crate doesn't export any native functions suitable for JNI? → The minimal `@jni/<crate-name>/` directory and wrapper project are **skipped**; no directory/project is created for that crate (only crates with JNI-suitable exports get a wrapper). C++ JNI glue (`rusty_lucene_jni.cpp`) will NOT include bindings for such crates.
- How does system handle conflicting JNI signatures (e.g., same function name with different Rust types)? → Uses `<crate-name>-<function-name>` as the Java method name (e.g., `core_search` → `RustyLuceneCore.search`) to avoid naming collisions.
- What if a Rust function uses complex types not easily mappable to Java (e.g., Rust enums with data payloads)? → Those functions are documented as "JNI-incompatible" in the wrapper (for applicable crates), with a fallback Java method that returns a generic representation or requires explicit conversion.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST automatically discover all Rust sub-crates under `@crates/` directory at runtime (without manual listing).
- **FR-002**: System MUST generate a Java JNI wrapper project under `@jni/<crate-name>/` for each Rust sub-crate that exports at least one native function suitable for JNI. (Only create wrapper for crates with JNI-suitable exports; skip for crates with no JNI-suitable exports)
- **FR-003**: System MUST generate minimum viable Java project structure including:
  - `src/main/java/<jni-package>/` with `*.java` source files
  - `build.gradle` (or `pom.xml`) with Java dependencies and JNI configuration
  - `jni/` directory with `AndroidNativeWrapper.java`-style JNI interface class (`<crate-name>-Jni`)
- **FR-004**: System MUST generate correct JNI method signatures that map Rust native function types to Java types (reasonable defaults: `i8,i16,i32,i64` → `byte/short/int/long`; `f32,f64` → `float/double`; `String`/`&str` → `java.lang.String`; `Vec<T>` → `java.util.List<?>`).
- **FR-005**: System MUST embed build configuration linking the JNI wrapper to the exact Rust native library output (e.g., `rusty-lucene-core-native`), based on the crate name and build output location.
- **FR-006**: System MUST provide an automated update mechanism. When a Rust sub-crate is modified and rebuilt, running the JNI wrapper update tool should regenerate only the corresponding Java JNI wrapper project with minimal manual steps (ideally just one command).
- **FR-007**: System MUST validate generated JNI wrapper code for basic correctness (detect obvious signature mismatches, e.g., mismatched parameter/return types, unsupported Rust types). Validation should fail on clear errors but allow reasonable defaults.
- **FR-008**: System MUST document JNI-incompatible Rust APIs in the wrapper (e.g., complex Rust types without natural Java equivalents). The documentation should be in `README.md` under `@jni/<crate-name>/` with clear guidance.

### Key Entities *(include if feature involves data)*

- **[CrateInfo]**: Represents a Rust sub-crate with identifier (crate name), location (`@crates/<crate-name>/`), list of native function exports suitable for JNI (func name, return type, param types, export status), dependencies on other crates.
- **[JniWrapper]**: Represents a Java JNI wrapper project under `@jni/<crate-name>/` with project structure, build config (`build.gradle`), JNI interface class (`<crate-name>-Jni`), and generated mapping of Rust native functions → Java methods.
- **[Mapping]**: Represents the bidirectional mapping from Rust native function → Java method signature (includes type conversion rules and naming).
- **[UpdateRecord]**: Records which Rust crate version/build a JNI wrapper corresponds to (crate name + build timestamp/hash), helping track consistency.

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
  Each criterion must be verifiable without knowing implementation details.
-->

### Measurable Outcomes

- **SC-001**: Given `@crates/` contains N Rust sub-crates with JNI-suitable native exports, the corresponding Java JNI wrapper project can be generated successfully. *(Verified by successfully generating and building wrapper projects for all applicable crates)*
- **SC-002**: 100% coverage of Rust sub-crates that export JNI-suitable native functions: Every crate in `@crates/` with at least one JNI-suitable native function must have a corresponding `@jni/<crate-name>/` wrapper project created (crates with no JNI-suitable exports are skipped; no directory is created). *(Verified by automated discovery scan that lists applicable crates and skips non-applicable ones)*
- **SC-003**: Generated JNI wrapper projects pass basic build validation: `./gradlew assemble` (or equivalent) succeeds without native linking errors. *(Verified by running build step)*
- **SC-04**: JNI wrapper integration test succeeds for ≥ 90% of native function exports from Rust sub-crates. *(Verified by running a simple Java test that calls each JNI method for each crate; allows <10% failure rate for genuinely non-mappable functions)*
- *(SC-05 removed since time-bound "≤ 30 minutes" update time is not required)*

## Assumptions

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right assumptions based on reasonable defaults
  chosen when the feature description did not specify certain details.
  -->

- [Assumption about target users, e.g., "Users have stable internet connectivity"] - Users have standard development environments with Rust toolchain and JDK 11+.
- [Assumption about scope boundaries, e.g., "Mobile support is out of scope for v1"] - No Android-specific code (e.g., no `AndroidNativeWrapper`-like classes for mobile); standard JVM/JNI used.
- [Assumption about data/environment, e.g., "Existing authentication system will be reused"] - The Rust native library output format is known and stable; no need to support version negotiation.
- [Dependency on existing system/service, e.g., "Requires access to the existing user profile API"] - Developers have write access to both `@crates/` and `@jni/` directories; no additional service dependencies for JNI wrapper generation.
- We do not need to support complex Rust types that have no natural Java equivalent (e.g., opaque Rust structs, raw pointers without safety guarantees); those will be documented as "JNI-incompatible".
- The feature does not need to generate runtime code; only static Java wrapper classes and project configuration files that link to the Rust native library.
