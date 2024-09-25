pub mod model;
use crate::ort_vad::silero;
use crate::ort_vad::utils::SampleRate;

use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use jni::objects::{JByteArray, JClass};
use jni::sys::{jfloat, jlong};
use crate::ort_vad::silero::Silero;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref SILERO_INSTANCE: Mutex<Option<Silero>> = Mutex::new(None);
}

#[no_mangle]
pub extern "system" fn Java_com_example_Vad_initSilero(env: JNIEnv) -> jlong {
    let silero = Silero::new(SampleRate::SixteenkHz, "").unwrap();
    let mut instance = SILERO_INSTANCE.lock().unwrap();
    *instance = Some(silero);
    Box::into_raw(Box::new(instance)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_example_Vad_processAudio<'a>(env: JNIEnv<'a>, _: JClass, audio_data: JByteArray<'a>) -> jfloat {
    let audio_bytes: Vec<u8> = env.convert_byte_array(&audio_data).unwrap();
    let audio_i16: Vec<i16> = audio_bytes.chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    let mut instance = SILERO_INSTANCE.lock().unwrap();
    
    if let Some(ref mut silero) = *instance {
        match silero.calc_level(&audio_i16) {
            Ok(level) => {
                // 根据 level 判断是否包含语音
                if level > 0.01 { // 设定一个阈值
                    return 1.0; // 包含语音
                }
            }
            Err(_) => {
                // 处理错误
            }
        }
    }
    0.0 // 不包含语音
}