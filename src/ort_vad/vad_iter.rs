use crate::ort_vad::{silero, utils};

use super::speech_state::{SpeechState, StreamState};

const DEBUG_SPEECH_PROB: bool = false;
#[derive(Debug)]
pub struct VadIter {
    silero: silero::Silero,
    pub params: Params,
    stream_state: StreamState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VadState {
    Silence,
    Start,
    Speaking,
    End,
}

impl VadIter {
    pub fn new(silero: silero::Silero, params: utils::VadParams) -> Self {
        let p = Params::from(params);
        let s_state = StreamState::new(&p);
        Self {
            silero,
            params: p,
            stream_state: s_state,
        }
    }

    pub fn process(&mut self, samples: &[i16]) -> Result<SpeechState, anyhow::Error> {
        let speech_prob: f32 = self.silero.calc_level(samples)?;
        let state = self.stream_state.update(&self.params, speech_prob);
        Ok(state)
    }

}

#[allow(unused)]
#[derive(Debug)]
pub struct Params {
    pub frame_size: usize,
    pub speech_threshold: f32,
    pub pre_speech_threshold_frame_cnt: usize,
    pub speech_threshold_frame_cnt: usize,
    pub silence_stop_frame_cnt: usize,
    pub max_speech_duration_s: f32,
    pub sample_rate: usize,
    pub sr_per_ms: usize,
    pub frame_size_samples: usize,
}

impl From<utils::VadParams> for Params {
    fn from(value: utils::VadParams) -> Self {
        let frame_size = value.frame_size;
        let threshold = value.threshold;
        let max_speech_duration_s = value.max_speech_duration_s;
        let sample_rate = value.sample_rate;
        let sr_per_ms = sample_rate / 1000;
        let frame_size_samples = frame_size * sr_per_ms;

        let silence_stop_frame_cnt = value.silence_stop_ms / frame_size;
        
        Self {
            frame_size,
            speech_threshold: threshold,
            pre_speech_threshold_frame_cnt: value.pre_speech_threshold_frame_cnt,
            speech_threshold_frame_cnt: value.speech_threshold_frame_cnt,
            silence_stop_frame_cnt,
            max_speech_duration_s,
            sample_rate,
            sr_per_ms,
            frame_size_samples,
        }
    }
}