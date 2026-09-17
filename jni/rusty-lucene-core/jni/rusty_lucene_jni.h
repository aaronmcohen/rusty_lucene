#ifndef RUSTY_LUCENE_JNI_H
#define RUSTY_LUCENE_JNI_H

#include <jni.h>

// These functions will be implemented by calling the Rust native functions
// Only JNI-suitable functions (c_int, c_float, c_char*, &str) are included
#define JNI_SEARCH "rusty_lucene_core_jni_search"
#define JNI_RANK "rusty_lucene_core_jni_rank"

extern jint rusty_lucene_core_jni_search(JNIEnv* env, jobject thiz, jstring query);
extern jfloat rusty_lucene_core_jni_rank(JNIEnv* env, jobject thiz, jint id, jfloat score);

#endif
