pub mod exposure;
pub mod ort_vad;
pub mod util;

use crate::exposure::{init_silero, process_audio};

#[cfg(target_os = "android")]
use jni::objects::{JByteArray, JClass};
#[cfg(target_os = "android")]
use jni::JNIEnv;
#[cfg(target_os = "android")]
use jni::sys::{jfloat, jlong};

#[cfg(any(target_os = "ios", target_os = "macos"))]
use std::os::raw::{c_float, c_void};

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_example_Vad_initSilero(_env: JNIEnv) -> jlong {
    init_silero() as jlong
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_example_Vad_processAudio<'a>(env: JNIEnv<'a>, _: JClass, audio_data: JByteArray<'a>) -> jfloat {
    let audio_bytes: Vec<u8> = env.convert_byte_array(&audio_data).unwrap();
    let audio_i16: Vec<i16> = audio_bytes.chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    process_audio(&audio_i16) as jfloat
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
