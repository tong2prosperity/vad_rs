pub mod model;
use std::ffi::c_long;
use crate::ort_vad::speech_state::SpeechState;
//use std::os::raw::c_long;
use crate::ort_vad::{silero::Silero, utils::VadParams};
use crate::ort_vad::utils::SampleRate;
use crate::ort_vad::vad_iter::{VadIter, VadState};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use log::debug;

#[repr(C)]
pub struct VadRes {
    pub talk_state: i32,
    pub err_code: i32,
    //pub msg: String,
}


lazy_static::lazy_static! {
    pub static ref SILERO_INSTANCE: Arc<Mutex<Option<Silero>>> = Arc::new(Mutex::new(None));
    // static iter
    pub static ref VAD_ITER: Arc<Mutex<Option<VadIter>>> = Arc::new(Mutex::new(None));

    static ref VAD_ITER_MAP: Mutex<HashMap<c_long, VadIter>> = Mutex::new(HashMap::new());
}

static mut NEXT_HANDLE: c_long = 0;

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
                return level;
            }
            Err(_) => {
                // 处理错误
            }
        }
    }
    0.0
}

pub fn process_vad_iter(handle: c_long, samples: &[i16]) -> VadRes {
    let mut map = VAD_ITER_MAP.lock().unwrap();
    let mut res = VadRes {
        talk_state: -1,
        err_code: 0,
        //msg: "".to_string(),
    };
    if let Some(vad_iter) = map.get_mut(&handle) {
        match vad_iter.process(samples) {
            Ok(v_state) => {
                res.talk_state = v_state as i32;
            }
            Err(e) => {
                res.err_code = -2;
                //res.msg = e.to_string();
            }
        }
    } else {
        res.err_code = -1;
        //res.msg = "Invalid VadIter handle".to_string();
    }
    res
}

pub fn init_vad_iter(param_str: &str) -> c_long {
    let mut param : VadParams = VadParams::default();
    if !param_str.is_empty() {

        match serde_json::from_str(param_str) {
            Ok(p) => param = p,
            Err(e) => {
                debug!("Error parsing JSON: {}", e);
            }
        }
    }
    let silero = Silero::new(SampleRate::SixteenkHz, "").unwrap();
    let vad_iter = VadIter::new(silero, param);
    
    let handle = unsafe {
        NEXT_HANDLE += 1;
        NEXT_HANDLE
    };
    
    VAD_ITER_MAP.lock().unwrap().insert(handle, vad_iter);
    handle
}

pub fn cleanup_vad_iter(handle: c_long) {
    VAD_ITER_MAP.lock().unwrap().remove(&handle);
}