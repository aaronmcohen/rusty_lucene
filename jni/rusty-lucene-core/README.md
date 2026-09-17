# JNI Wrapper: rusty-lucene-core-jni

Java Native Interface (JNI) wrapper for the `rusty-lucene-core` Rust crate.

## Structure

```
src/
  main/
    java/com/example/rclucene/jni/RustyLuceneJni.java
    cpp/rusty_lucene_jni.cpp       # JNI glue code
  build.gradle                       # Gradle build script

build/
  native/rusty_lucene_jni.dylib    # Dynamic library on macOS, .so on Linux
```

## JNI-suitable APIs

The following Rust native functions are mapped to Java via JNI:

- `search(query: *const c_char, len: c_int)` → `public int search(String query)`  
- `rank(id: c_int, score: c_float)` → `public float rank(int id, float score)`

## JNI-incompatible APIs

The following Rust types do not have natural Java equivalents and are documented as JNI-incompatible:

- `get_complex_enum(state: *const c_char)` → No Java JNI method is created for this. It is documented in `contracts/jni-contract.md` and `README.md` as JNI-incompatible (enum with payload has no natural Java equivalent). No fallback Java method is provided — callers should not attempt to call it via JNI.

When Rust enums with data payloads are used in native function signatures (e.g., `GetComplexEnum`), they are documented as JNI-incompatible. No wrapper directory or JNI glue is generated for such functions.

JNI-incompatible types require significant custom serialization/mapping and are not automatically converted.

## Build

Build the JNI wrapper native library and package:

```bash
# or
./gradlew nativeCompile
```

On macOS, this produces `build/native/rusty_lucene_jni.dylib`.  
On Linux, this produces `build/native/rusty_lucene_jni.so`.

## Test

Run the Java test that loads the native library and calls JNI methods:

```bash
./gradlew nativeTest
```

The test sets `LD_LIBRARY_PATH` / `DYLD_LIBRARY_PATH` automatically to load the native library.

## Integration with Rust

- The wrapper links against `librusty_lucene_core.dylib` / `librusty_lucene_core.so` from `target/release/`.
- No copy-pasting of Rust source is needed; only native library linking is required.
- The JNI glue (`java_lucene_jni.cpp`) only exports `jniSearch` and `jniRank` — JNI-incompatible functions (`get_complex_enum`) are NOT included in the glue.

## Quick Validation

```bash
# Build Rust native library
cargo build --release

# Build JNI wrapper
cd jni/rusty-lucene-core
./gradlew nativeCompile

# Run validation test
./gradlew nativeTest
```

Expected output: all tests pass, native library loads successfully.
