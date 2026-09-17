1: # Implementation Plan

2: ## Technical Context

3: Based on the feature spec, the following entities and requirements need to be implemented:

4: - Feature name: JNI Wrappers
- Description: Create a `@jni/` directory containing JNI glue C++ (`src/main/cpp/rusty_lucene_jni.cpp`), header (`jni/rusty_lucene_jni.h`), Java wrapper (`src/main/java/com/example/rclucene/jni/RustyLuceneJni.java`), `build.gradle`, `README.md`. Only create `@jni/<crate-name>/` wrapper directories for Rust sub-crates that export at least one JNI-suitable native function (skip crates with only JNI-incompatible functions like `get_complex_enum`).

5: ## Constitution Check

6: The constitution requires that certain gates must not be violated:
  - No runtime code generation (e.g., compiling Java at runtime) — this gate is not violated because only static Java wrapper classes and Gradle build script are created.
  - JNI glue must link against the Rust native library only — satisfied by using `-lrusty_lucene_core` linker flag.

7: ## Phase 0: Research

8: Resolve concrete resolution steps for known unknowns:
  - Rust exported function names: Inspect `rusty-lucene-core/src/lib.rs` for `#[no_mangle] pub extern "C"` functions (`search`, `rank`, `get_complex_enum`).
  - Platform-specific build flags: Run `cargo build --release` on both macOS and Linux (or check flags); macOS uses `.dylib` with `-dynamiclib`, Linux uses `.so` with `-shared`. Record flags in `build.gradle` template.
  - JNI-incompatible type pattern: Use clear rule from `contracts/jni-type-rules.md` (Rust enums with data payloads = incompatible; basic primitives = compatible). Define a simple heuristic scanner in `jni-update` script.

9: ## Phase 1: Design & Contracts

10: After research, define:
    - `data-model.md`: Model the JNI wrapper entities (e.g., `RustyLuceneJni` class, `jniGlue` struct), fields, validation rules.
    - `/contracts/`: Document the JNI interface contract (e.g., `jniSearch(JNIEnv, String) -> int`, `jniRank(JNIEnv, int, float) -> float`, `jniGetComplexEnum(JNIEnv, int) -> String`). Include a note about JNI-incompatible types being documented rather than exported.
    - `jni-type-rules.md`: Define clear "compatible" vs "incompatible" type rules (see `contracts/jni-type-rules.md`).
    - `quickstart.md`: Runnable validation guide with prerequisites (M1/Intel Mac or Linux with GCC/Clang, JDK), setup commands (`cargo build --release`, `make jni-build`), test/run commands (`make jni-test`), expected outcomes (native library loads, tests pass).

11: ## Phase 2: Foundational (Blocking Prerequisites)

12: **Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

13: **⚠️ CRITICAL**: No user story work can begin until this phase is complete

14: - [P] T005 [Phase 0] Verify Rust native exports — Run `cargo build --release` to produce `target/release/librusty_lucene_core.{so/dylib}`; inspect `#[no_mangle] pub extern "C"` functions in `rusty-lucene-core` (`search`, `rank`, `get_complex_enum`).
    - Run `./gradlew nativeCompile` — should succeed without linker errors. Record linker flags (`-dynamiclib` for macOS, `-shared` for Linux).
    - On macOS: `-dynamiclib`; on Linux: `-shared`; verify the linker can find `librusty_lucene_core` from `target/release/`.
    - **Verification**: Run `jni-validate` script that scans `@crates/` for JNI-suitable exports and verifies no `@jni/<crate-name>/` directory is created for crates with only JNI-incompatible exports (e.g., `get_complex_enum`).
    - [P] T010 [Phase 0] Verify JNI-incompatible type heuristic — Run the `jni-update` script's detection on `@crates/`; confirm it correctly classifies `search`/`rank` as compatible and `get_complex_enum` as incompatible. No `@jni/<crate-name>/` directory should be created for crates with only incompatible exports.

    - [P] T004 [Phase 0] Add `jni-*` targets (`jni-build`, `jni-test`, `jni-clean`) to top-level `Makefile` (if not already present) — `jni-build` does `cargo build --release` + `./gradlew nativeCompile`, `jni-test` does `jni-build` + `./gradlew nativeTest`, `jni-clean` removes `build/native/*.dylib/*.so` and `build/libs/*`.

15: ## Done When

16: - [ ] `research.md` complete (all resolution steps done: function names known, platform flags confirmed, type rule verified)
    - [ ] `data-model.md`, `/contracts/`, `jni-type-rules.md`, `quickstart.md` generated and reviewed
    - [ ] `plan.md` reports branch `feat/002-jni-wrappers` and generated artifacts (`@jni/rusty-lucene-core/`, `Makefile` with `jni-*` targets)

17: ## Additional Validation Steps

18: - [P] T021 [Phase 1] Write `contracts/jni-type-rules.md` — define clear "compatible" vs "incompatible" type rules per the `jni-update` script heuristic.
    - [P] T022 [Phase 1] Review JNI-integration test failures — when `make jni-test` fails, classify failures as "fixable" (e.g., build configuration issue) or "JNI-INCOMPATIBLE" (e.g., genuinely non-mappable function like `get_complex_enum`). For JNI-INCOMPATIBLE functions, document a fallback Java method (`getComplexEnum(int) → String`) in `README.md`.
    - [P] T001-US0 [Phase 0/AM2] Create minimal `jni-update` script that:
        - Scans `@crates/` for `#[no_mangle] pub extern "C"` native exports
        - Uses `jni-type-rules.md` rule to determine which crates have JNI-suitable exports (simple whitelist: `c_int`, `c_float`, `*const c_char`, `&str`)
        - Skips crates without JNI-suitable exports (e.g., `get_complex_enum` with enum payload — documented as JNI-incompatible)
        - Regenerates `@jni/<crate-name>/` structure only for applicable crates
