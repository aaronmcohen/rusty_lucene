# JNI Interface Contracts for JNI Wrappers

## Contracts ( JNI-compatible only; JNI-incompatible APIs are documented in README.md)

| Method | JNI Signature | Description | JNI-compatible? | Rust Export |
|---------|---------------|-------------|-----------------|-------------|
| search  | search(JNIEnv, String) → int     | Executes search on the index; returns hit count. | yes | #[no_mangle] pub extern "C" fn search(env: &JNIEnv, query: &str) -> i32 |
| rank    | rank(JNIEnv, long, double) → int| Computes similarity/rank score for a document. | yes | #[no_mangle] pub extern "C" fn rank(env: &JNIEnv, doc_id: i64, query_weight: f32) -> i32 |
| get_complex_enum | getComplexEnum(JNIEnv, int) → String | Retrieves a complex enum value. | no (*) | #[no_mangle] pub extern "C" fn get_complex_enum(env: &JNIEnv, id: i32) -> *const ComplexEnum |

### Notes:
- (*) JNI-incompatible: Complex enum with payload → documented in README.md; fallback Java method `getComplexEnum(int)` returns serialized String representation.
- Type mapping follows heuristic: primitives map directly (i32→int, f32→float), String↔java.lang.String, Vec<primitive>→java.util.List<?>.
- `JNIEnv` is implicit (passed by JNI runtime); no method needs to accept `JNIEnv` explicitly in Java code (Java methods call into native linker-linked library).
- Method names use camelCase (e.g., `search`, `rank`, `getComplexEnum`) matching typical Java conventions.
- Resolution of Needs Clarification: Function names (`search`, `rank`, `get_complex_enum`) confirmed by inspecting `rusty-lucene-core` native exports; platform flags (`-dynamiclib` for macOS, `-shared` for Linux) applied in `build.gradle`; JNI-incompatible type rule defined.
