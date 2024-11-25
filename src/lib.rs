pub mod exposure;
pub mod ort_vad;
pub mod util;

use crate::exposure::{init_silero, process_audio, VadRes, init_vad_iter, process_vad_iter, cleanup_vad_iter};
use std::ffi::c_long;

#[cfg(target_os = "android")]
use jni::objects::{JByteArray, JClass, JString};
#[cfg(target_os = "android")]
use jni::JNIEnv;
#[cfg(target_os = "android")]
use jni::sys::{jfloat, jlong};

use std::ffi::{c_ulong, c_char};
#[cfg(any(target_os = "ios", target_os = "macos"))]
use std::os::raw::{c_float, c_void};


#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_qktz_kimivad_KimiVad_init_1vad_1iter<'a>(mut env: JNIEnv<'a>, _: JClass, param_str: JString<'a>) -> jlong {
    let param_str:String = env.get_string(&param_str).expect("Failed to get string").into();
    init_vad_iter(&param_str) as jlong
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_qktz_kimivad_KimiVad_process_1vad_1iter<'a>(env: JNIEnv<'a>, _: JClass, handle: jlong, audio_data: JByteArray<'a>) -> jlong {
    let audio_bytes: Vec<u8> = env.convert_byte_array(&audio_data).unwrap();
    let audio_i16: Vec<i16> = audio_bytes.chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();
    let res = process_vad_iter(handle, &audio_i16);
    let real_res = if res.err_code == 0 {
        res.talk_state as jlong
    } else {
        -1
    };
    real_res
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_qktz_kimivad_KimiVad_cleanup_1vad_1iter<'a>(env: JNIEnv<'a>, _: JClass, handle: jlong) {
    cleanup_vad_iter(handle);
}


#[cfg(any(target_os = "ios", target_os = "macos"))]
#[no_mangle]
pub extern "C" fn init_silero_apple() -> *mut c_void {
    init_silero() as *mut c_void
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[no_mangle]
pub extern "C" fn process_audio_apple(audio_data: *const i16, audio_len: usize) -> c_float {
    let audio_slice = unsafe { std::slice::from_raw_parts(audio_data, audio_len) };
    process_audio(audio_slice) as c_float
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[no_mangle]
pub extern "C" fn init_vad_iter_apple(param_str: *const c_char) -> c_long {
    let param_str = unsafe { std::ffi::CStr::from_ptr(param_str) }.to_string_lossy().into_owned();
    init_vad_iter(&param_str) 
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[no_mangle]
pub extern "C" fn process_vad_iter_apple(handle: c_long, audio_data: *const i16, audio_len: usize) -> VadRes {
    // check the audio length, the sample rate is 24000, so the length should be 24000 * 0.02 = 480

    let audio_slice = unsafe { std::slice::from_raw_parts(audio_data, audio_len) };
    process_vad_iter(handle, audio_slice)
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[no_mangle]
pub extern "C" fn cleanup_vad_iter_apple(handle: c_long) {
    cleanup_vad_iter(handle);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
