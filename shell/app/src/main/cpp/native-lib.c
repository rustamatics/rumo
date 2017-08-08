#include <jni.h>
#include <string.h>
#include <stdint.h>

int32_t entry_point();

JNIEXPORT jstring JNICALL
Java_com_droid_simple_MainActivity_stringFromJNI( JNIEnv* env,
                                                      jobject thiz )
{
  entry_point();
  return (*env)->NewStringUTF(env, "Hello from JNI !");
}
