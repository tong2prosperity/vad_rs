pub mod exposure;
pub mod ort_vad;
pub mod util;

use jni::objects::{JByteArray, JClass};
use jni::JNIEnv;
use jni::sys::{jfloat, jlong};
use crate::exposure::{init_silero, process_audio};

#[no_mangle]
pub extern "system" fn Java_com_example_Vad_initSilero(_env: JNIEnv) -> jlong {
    init_silero() as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_example_Vad_processAudio<'a>(env: JNIEnv<'a>, _: JClass, audio_data: JByteArray<'a>) -> jfloat {
    let audio_bytes: Vec<u8> = env.convert_byte_array(&audio_data).unwrap();
    let audio_i16: Vec<i16> = audio_bytes.chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    process_audio(&audio_i16) as jfloat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
