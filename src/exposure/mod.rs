pub mod model;
use crate::ort_vad::silero::Silero;
use crate::ort_vad::utils::SampleRate;
use std::sync::{Mutex, Arc};

lazy_static::lazy_static! {
    pub static ref SILERO_INSTANCE: Arc<Mutex<Option<Silero>>> = Arc::new(Mutex::new(None));
}

// 移除了JNI相关的函数

pub fn init_silero() -> *mut Arc<Mutex<Option<Silero>>> {
    let silero = Silero::new(SampleRate::SixteenkHz, "").unwrap();
    let mut instance = SILERO_INSTANCE.lock().unwrap();
    *instance = Some(silero);
    Box::into_raw(Box::new(SILERO_INSTANCE.clone()))
}

pub fn process_audio(audio_i16: &[i16]) -> f32 {
    if audio_i16.is_empty() {
        return -1.0;
    }
    if audio_i16.len() < 512 {
        return -1.0;
    }


    let mut instance = SILERO_INSTANCE.lock().unwrap();
    
    if let Some(ref mut silero) = *instance {
        match silero.calc_level(audio_i16) {
            Ok(level) => {
                if level > 0.01 {
                    return 1.0;
                }
            }
            Err(_) => {
                // 处理错误
            }
        }
    }
    0.0
}