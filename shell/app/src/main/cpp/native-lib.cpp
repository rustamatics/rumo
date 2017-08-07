#include <jni.h>
#include <string>
#include <stdint.h>

int32_t entry_point();

extern "C"
JNIEXPORT jstring JNICALL
Java_com_droid_simple_MainActivity_stringFromJNI(
        JNIEnv* env,
        jobject /* this */) {
    entry_point();
    std::string hello = "Connecting to backend...";
    return env->NewStringUTF(hello.c_str());
}
