use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy)]
pub enum SampleRate {
    EightkHz,
    SixteenkHz,
}

impl From<SampleRate> for i64 {
    fn from(value: SampleRate) -> Self {
        match value {
            SampleRate::EightkHz => 8000,
            SampleRate::SixteenkHz => 16000,
        }
    }
}

impl From<SampleRate> for usize {
    fn from(value: SampleRate) -> Self {
        match value {
            SampleRate::EightkHz => 8000,
            SampleRate::SixteenkHz => 16000,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct VadParams {
    pub frame_size: usize,
    pub threshold: f32,
    pub silence_stop_ms: usize,
    pub max_speech_duration_s: f32,
    pub sample_rate: usize,
    pub pre_speech_threshold_frame_cnt: usize,
    pub speech_threshold_frame_cnt: usize,
}

impl Default for VadParams {
    fn default() -> Self {
        Self {
            frame_size: 32,
            threshold: 0.5,
            silence_stop_ms: 1200,
            max_speech_duration_s: f32::INFINITY,
            sample_rate: 16000,
            pre_speech_threshold_frame_cnt: 1,
            speech_threshold_frame_cnt: 1,
        }
    }
}

#[derive(Debug, Default)]
pub struct TimeStamp {
    pub start: i64,
    pub end: i64,
}

impl std::fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[start:{:08}, end:{:08}]", self.start, self.end)
    }
}
