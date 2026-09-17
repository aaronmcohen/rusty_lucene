package com.example.rclucene.jni;

import androidx.annotation.NonNull;

/**
 * JNI wrapper for rusty-lucene-core native functions.
 *
 * Maps Rust native function signatures to Java types:
 *   c_char[], len -> java.lang.String, int
 *   c_int -> int, jint
 *   c_float -> float, jfloat
 *   c_int -> int for enum-like returns (JNI-incompatible types are documented, not exported)
 */
public final class RustyLuceneJni {

    private static volatile RustyLuceneJni instance;

    private final long nativePtr;

    private RustyLuceneJni(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    public static RustyLuceneJni getInstance() {
        if (instance == null) {
            instance = new RustyLuceneJni(jniInit());
        }
        return instance;
    }

    private static native int jniSearch(JNIEnv env, jstring query);
    private static native float jniRank(JNIEnv env, jint id, jfloat score);
    // JNI-incompatible: jniGetComplexEnum for get_complex_enum is NOT exported (documented only)

    /**
     * Search for a query in the index.
     *
     * @param query the query string (UTF-8 encoded)
     * @return number of matching fields (>=0), 0 if no match
     */
    public int search(String query) {
        if (query == null) return 0;
        return jniSearch(null, query);
    }

    /**
     * Rank a document given its ID and raw score.
     *
     * @param id document ID (int)
     * @param score raw score from scoring function (float)
     * @return normalized rank (float, in [0.0, 1.0] range for typical use)
     */
    public float rank(int id, float score) {
        return jniRank(null, id, score);
    }

    // JNI-incompatible: getComplexEnum for get_complex_enum with enum payload is documented only,
    // not exported via JNI. See README.md or contract for documentation.
    // No Java method is provided for get_complex_enum - it's genuinely non-mappable.

    private static long jniInit() {
        // JNIInit returns a pointer to instance data; we just return 0 for now
        // The real implementation would register functions and return a pointer
        return 0L;
    }

    private native long jniBind(JNIEnv env, long userData); // placeholder
}
