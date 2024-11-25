#include <jni.h>
#include <string>
#include <iostream>

extern "C" JNIEXPORT jstring JNICALL
Java_com_qktz_vad_1rs_MainActivity_stringFromJNI(
        JNIEnv* env,
        jobject /* this */) {
    std::string hello = "Hello from C++";
    return env->NewStringUTF(hello.c_str());
}
extern "C"
JNIEXPORT jlong JNICALL
Java_com_qktz_kimivad_KimiVad_init_1vad_1iter(JNIEnv *env, jclass clazz, jstring params) {
    // TODO: implement init_vad_iter()
    std::cout << "fuck" << std::endl;
}