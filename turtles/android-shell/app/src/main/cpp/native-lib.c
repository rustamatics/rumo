#include <string.h>
#include <jni.h>

#include <android/log.h>
#include <android_native_app_glue.h>

#define LOGI(...) ((void)__android_log_print(ANDROID_LOG_INFO, "native-lib", __VA_ARGS__))
#define LOGW(...) ((void)__android_log_print(ANDROID_LOG_WARN, "native-lib", __VA_ARGS__))

// extern "C" {
extern void rust_android_main(struct android_app* state);
// }


jstring bridge_package_name(JNIEnv* env,jobject entryObject) {

    jclass android_content_Context =(*env)->GetObjectClass(env, entryObject);
    //or use FindClass

    jmethodID midGetPackageName = (*env)->GetMethodID(env,
                                                      android_content_Context,
                                                      "getPackageName",
                                                      "()Ljava/lang/String;");

    jstring packageName=(*env)->CallObjectMethod(env, entryObject, midGetPackageName);
    return packageName;
}

/**
 * This is the main entry point of a native application that is using
 * android_native_app_glue.  It runs in its own thread, with its own
 * event loop for receiving input events and doing other things.
 */
void android_main(struct android_app* state) {
    rust_android_main(state);
}
