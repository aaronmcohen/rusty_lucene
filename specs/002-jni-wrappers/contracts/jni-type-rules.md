# JNI Incompatible Type Rules

This document defines what Rust types are considered "genuinely non-mappable" (JNI-incompatible) vs "compatible" (JNI-suitable). A type is compatible if it can be naturally mapped to Java without complex conversion or loss of information. A type is incompatible if it has no natural Java equivalent or requires significant manual conversion.

## Compatible Types (JNI-suitable)

- Primitive integer types used in native functions: `c_int`, `c_long`, etc. → `int`, `long` in Java
- Primitive floating-point types used in native functions: `c_float`, `c_double`, `f32`, `f64` → `float`, `double` in Java
- Character pointers without complex data: `*const c_char`, `*mut c_char` (null-terminated strings only) → `java.lang.String` in Java
- String references (`&str`/`String`) used as simple input/output without enum payloads → `java.lang.String` in Java
- Simple collections of compatible primitives: `Vec<i32>`, `Vec<f32>` → `java.util.List<Integer>` or `java.util.List<Float>` in Java (marshaled as list)
- Fixed-size arrays of compatible primitives (when size is known or marshaled) → appropriate Java collection type

## Incompatible Types (JNI-incompatible)

- Rust enums with data payloads (i.e., variants that carry non-primitive values):
  - Example: `enum RankMode { Start { scale: f32 }, Exact { value: i32 } }` → No natural Java equivalent without custom serialization
  - Example: `enum AnalysisResult { Success { metadata: HashMap<String, String> }, Failure { error: String } }` → Documentation only; no JNI binding
  - Any enum variant embedding structs, other enums, or complex data → Incompatible
- Opaque pointers or raw pointers without safety guarantees (`*mut T`, `*const T` where T is a complex struct) → Incompatible
- Complex nested structs with runtime-sized fields → Incompatible (would require Java object mapping)
- Function types or callbacks → Incompatible (would require JNI interface setup)

## How to detect in the `jni-update` script

1. Scan `@crates/<crate-name>/src/lib.rs` for `#[no_mangle] pub extern "C"` functions.
2. For each exported function signature:
   - If the return/param type is `c_int`, `c_long`, `c_float`, `c_double`, `*const c_char`, `&str`, or `String`, mark it as JNI-suitable/compatible.
   - If the return/param type is `enum <Name>` where `<Name>` has any variant that contains data (e.g., `Start { scale: f32 }`, `Exact { value: i32 }`), mark the function as JNI-incompatible.
   - If the type is a complex struct (e.g., containing other structs), mark as incompatible.
3. A crate is "applicable" (gets a `@jni/<crate-name>/` wrapper) only if it has at least one JNI-suitable export. Crates with only JNI-incompatible exports (e.g., `get_complex_enum` with enum payload) are skipped — no `@jni/<crate-name>/` directory is created for them.

## Update for SC-04 wording

"JNI wrapper integration test succeeds for ≥ 90% of native function exports from Rust sub-crates."

→ "JNI wrapper integration test succeeds for ≥ 90% of native function exports from Rust sub-crates that are JNI-compatible. The test allows <10% failure rate for genuinely non-mappable functions (e.g., functions using Rust enums with data payloads)."
