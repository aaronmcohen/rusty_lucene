#include <jni.h>
#include <cstring>  // for strlen

extern "C" {
    extern int32_t search(const char* query, int32_t len);
    extern float rank(int32_t id, float score);
    // Note: get_complex_enum is JNI-incompatible (uses enum with data payload)
    // and is NOT included in the JNI glue
}

JNIEXPORT jint JNICALL Java_com_example_rclucene_jni_RustyLuceneJni_jniSearch(JNIEnv* env, jobject thiz, jstring query) {
    if (query == nullptr) return 0;
    
    jboolean isCopy = JNI_FALSE;
    const char* utf8 = env->GetStringUTFChars(query, &isCopy);
    if (!utf8) return 0;
    
    int len = strlen(utf8) + 1;  // +1 for null terminator
    
    int result = search(utf8, len);
    env->ReleaseStringUTFChars(query, utf8);
    
    return (jint)result;
}

JNIEXPORT jfloat JNICALL Java_com_example_rclucene_jni_RustyLuceneJni_jniRank(JNIEnv* env, jobject thiz, jint id, jfloat score) {
    int32_t id32 = (int32_t)id;
    float result = rank(id32, score);
    return (jfloat)result;
}

JNIEXPORT jlong JNICALL Java_com_example_rclucene_jni_RustyLuceneJni_jniInit(JNIEnv* env, jobject thiz) {
    return 0LL;
}

JNIEXPORT jlong JNICALL Java_com_example_rclucene_jni_RustyLuceneJni_jniBind(JNIEnv* env, jobject thiz, long userData) {
    return userData;
}
