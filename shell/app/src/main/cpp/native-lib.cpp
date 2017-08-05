#include <jni.h>
#include <string>

extern "C"
JNIEXPORT jstring JNICALL
Java_com_droid_simple_MainActivity_stringFromJNI(
        JNIEnv* env,
        jobject /* this */) {
    std::string hello = "Connecting to backend...";
    return env->NewStringUTF(hello.c_str());
}
