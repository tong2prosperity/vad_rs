use std::u32;

use super::{utils, vad_iter::Params};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SpeechState {
    Silent,        // 初始静音状态
    StartSpeaking, // 开始说话
    Speaking,      // 持续说话中
    TempSilence,   // 临时静音
    StopSpeaking,  // 结束说话
}

#[derive(Debug)]
pub struct StreamState {
    sample_per_frame: u32, // 设定后不可改
    speech_state: SpeechState,
    temp_silence_cnt: u32,      // 临时静音计数
    silence_threshold_cnt: u32, // 多少次连续静音才能认为是要结束了 设定后不可改
    speech_cnt: u32,            // 记录说话持续时间
    last_speech_ms: f32,        // 记录上一次说话时长 count by ms
    current_speech: utils::TimeStamp,
    speeches: Vec<utils::TimeStamp>,
    pre_speech_threshold_cnt: u32, // 多少次连续说话才能认为是要说话了 设定后不可改
    speech_threshold_cnt: u32,     // 多少次连续说话才能认为是真正说话了 设定后不可改

    speech_threshold: f32,   // 设定后不可改
    low_prob_threshold: f32, // 设定后不可改
    max_speech_cnt: u32,
}

impl StreamState {
    pub fn new(params: &Params) -> Self {
        Self {
            sample_per_frame: params.frame_size_samples as u32,
            speech_state: SpeechState::Silent,
            temp_silence_cnt: 0,
            silence_threshold_cnt: params.silence_stop_frame_cnt as u32,
            speech_cnt: 0,
            pre_speech_threshold_cnt: params.pre_speech_threshold_frame_cnt as u32,
            speech_threshold_cnt: params.speech_threshold_frame_cnt as u32,
            last_speech_ms: 0.0,
            current_speech: utils::TimeStamp::default(),
            speeches: Vec::new(),
            speech_threshold: params.speech_threshold.max(0.3),
            low_prob_threshold: params.speech_threshold.max(0.3) - 0.15,
            max_speech_cnt: u32::MAX,
        }
    }

    pub fn finish_round(&mut self, record: bool) {
        self.speech_state = if !record {
            SpeechState::Silent
        } else {
            SpeechState::StopSpeaking
        };

        self.temp_silence_cnt = 0;
        if record {
            self.last_speech_ms =
                self.speech_cnt as f32 * self.sample_per_frame as f32 / 16000.0 * 1000.0;
        }
        self.speech_cnt = 0;
    }

    pub fn update(&mut self, params: &Params, speech_prob: f32) -> SpeechState {
        let is_speech = speech_prob > self.speech_threshold;
        let is_low_prob = speech_prob > self.low_prob_threshold;

        match self.speech_state {
            SpeechState::Silent => {
                if is_speech {
                    self.speech_cnt += 1;
                    if self.speech_cnt >= self.pre_speech_threshold_cnt {
                        self.speech_state = SpeechState::StartSpeaking;
                    }
                } else {
                    self.speech_cnt = 0;
                }
            }

            SpeechState::StartSpeaking => {
                if is_low_prob {
                    self.speech_cnt += 1;
                    if self.speech_cnt >= self.speech_threshold_cnt + self.pre_speech_threshold_cnt
                    {
                        self.speech_state = SpeechState::Speaking;
                    }
                } else {
                    // 可能是误触发，回到静音状态
                    self.speech_state = SpeechState::Silent;
                    self.finish_round(false);
                }
            }

            SpeechState::Speaking => {
                if is_low_prob {
                    // 继续说话
                    self.speech_cnt += 1;
                    if self.speech_cnt > self.max_speech_cnt {
                        // 超过最大说话时长，强制结束
                        self.finish_round(true);
                    }
                } else {
                    // 开始临时静音
                    self.speech_state = SpeechState::TempSilence;
                    self.temp_silence_cnt += 1;
                }
            }

            SpeechState::TempSilence => {
                if is_low_prob {
                    // 重新开始说话
                    self.speech_state = SpeechState::Speaking;
                    self.speech_cnt += 1;
                    self.temp_silence_cnt = 0;
                } else {
                    // 继续静音
                    self.temp_silence_cnt += 1;
                    if self.temp_silence_cnt >= self.silence_threshold_cnt {
                        self.finish_round(true);
                    }
                }
            }

            SpeechState::StopSpeaking => {
                self.speech_state = SpeechState::Silent;
                if is_speech {
                    self.speech_cnt += 1;
                }
            }
        }

        self.speech_state
    }

    pub fn get_last_speech_ms(&self) -> f32 {
        self.last_speech_ms
    }

    pub fn get_state(&self) -> SpeechState {
        self.speech_state
    }

    pub fn debug_print_speeches(&self) {
        for speech in &self.speeches {
            println!("Speech: {:?}", speech);
        }
    }
}

#[cfg(test)]
mod tests {
    use utils::VadParams;

    use super::*;
    #[test]
    fn test_initial_state() {
        let params = Params::from(VadParams::default());
        let stream_state = StreamState::new(&params);
        assert_eq!(stream_state.get_state(), SpeechState::Silent);
    }

    #[test]
    fn test_start_speaking() {
        let params = Params::from(VadParams::default());
        let mut stream_state = StreamState::new(&params);

        // 模拟连续3次超过低概率阈值
        for _ in 0..3 {
            stream_state.update(&params, 0.55);
        }

        assert_eq!(stream_state.get_state(), SpeechState::StartSpeaking);
    }

    #[test]
    fn test_speaking() {
        let params = Params::from(VadParams::default());
        let mut stream_state = StreamState::new(&params);

        // 模拟连续3次超过低概率阈值
        for _ in 0..7 {
            stream_state.update(&params, 0.55);
        }

        // 确认继续说话
        stream_state.update(&params, 0.55);

        assert_eq!(stream_state.get_state(), SpeechState::Speaking);
    }

    #[test]
    fn test_temp_silence() {
        let params = Params::from(VadParams::default());

        let mut stream_state = StreamState::new(&params);

        // 模拟连续3次超过低概率阈值
        for _ in 0..6 {
            stream_state.update(&params, 0.55);
        }

        // 确认继续说话
        stream_state.update(&params, 0.55);

        // 进入临时静音状态
        stream_state.update(&params, 0.3);

        assert_eq!(stream_state.get_state(), SpeechState::TempSilence);
    }

    #[test]
    fn test_resume_speaking() {
        let params = Params::from(VadParams::default());
        let mut stream_state = StreamState::new(&params);

        // 模拟连续3次超过低概率阈值
        for _ in 0..3 {
            stream_state.update(&params, 0.55);
        }

        // 确认继续说话
        stream_state.update(&params, 0.55);

        // 进入临时静音状态
        stream_state.update(&params, 0.45);

        // 重新开始说话
        stream_state.update(&params, 0.55);

        assert_eq!(stream_state.get_state(), SpeechState::Speaking);
    }

    #[test]
    fn test_stop_speaking() {
        let params = Params::from(VadParams::default());
        let mut stream_state = StreamState::new(&params);

        // 模拟连续3次超过低概率阈值
        for _ in 0..3 {
            stream_state.update(&params, 0.55);
        }

        // 确认继续说话
        stream_state.update(&params, 0.55);

        // 进入临时静音状态
        stream_state.update(&params, 0.3);

        // 模拟连续3次静音
        for _ in 0..3 {
            stream_state.update(&params, 0.3);
        }

        assert_eq!(stream_state.get_state(), SpeechState::StopSpeaking);
    }

    #[test]
    fn test_return_to_silent() {
        let params = Params::from(VadParams::default());
        let mut stream_state = StreamState::new(&params);

        // 模拟连续3次超过低概率阈值
        for _ in 0..3 {
            stream_state.update(&params, 0.55);
        }

        // 确认继续说话
        stream_state.update(&params, 0.55);

        // 进入临时静音状态
        stream_state.update(&params, 0.3);

        // 模拟连续3次静音
        for _ in 0..4 {
            stream_state.update(&params, 0.3);
        }

        

        assert_eq!(stream_state.get_state(), SpeechState::Silent);
    }

    // #[test]
    // fn test_max_speech_duration() {
    //     let params = Params::from(VadParams::default());
    //     let mut stream_state = StreamState::new(&params);

    //     // 模拟连续3次超过低概率阈值
    //     for _ in 0..3 {
    //         stream_state.update(&params, 0.55);
    //     }

    //     // 确认继续说话
    //     stream_state.update(&params, 0.55);

    //     // 模拟超过最大说话时长
    //     for _ in 0..1000 {
    //         stream_state.update(&params, 0.55);
    //     }

    //     assert_eq!(stream_state.get_state(), SpeechState::StopSpeaking);
    // }

    #[test]
    fn test_false_trigger() {
        let params = Params::from(VadParams::default());
        let mut stream_state = StreamState::new(&params);

        // 模拟连续3次超过低概率阈值
        for _ in 0..3 {
            stream_state.update(&params, 0.55);
        }

        // 确认继续说话
        stream_state.update(&params, 0.3); // 误触发

        assert_eq!(stream_state.get_state(), SpeechState::Silent);
    }

    #[test]
    fn test_record_last_speech_duration() {
        let params = Params::from(VadParams::default());
        let mut stream_state = StreamState::new(&params);

        // 模拟连续3次超过低概率阈值
        for _ in 0..3 {
            stream_state.update(&params, 0.55);
        }

        // 确认继续说话
        stream_state.update(&params, 0.55);

        // 模拟说话1000帧
        for _ in 0..1000 {
            stream_state.update(&params, 0.55);
        }

        // 进入临时静音状态
        stream_state.update(&params, 0.3);

        // 模拟连续3次静音
        for _ in 0..3 {
            stream_state.update(&params, 0.3);
        }

        // 确认结束说话
        //stream_state.finish_round(true);

        // 计算上一次说话时长
        let expected_duration = (64 * 1004) as f32; // 1000 frames * 160 samples per frame / 1000 ms
        assert_eq!(stream_state.get_last_speech_ms().round(), expected_duration.round());
    }
}
