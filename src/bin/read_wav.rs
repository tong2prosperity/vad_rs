use rust_vad::ort_vad::*;

fn main() {

    // let model_path = std::env::var("SILERO_MODEL_PATH")
    //     .unwrap_or_else(|_| String::from("./model/silero_vad.onnx"));
    let audio_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("/Users/nixi/Documents/kimi_audio/audio_file/dingding16k.wav"));
    let mut wav_reader = hound::WavReader::open(audio_path).unwrap();
    let sample_rate = match wav_reader.spec().sample_rate {
        8000 => utils::SampleRate::EightkHz,
        16000 => utils::SampleRate::SixteenkHz,
        _ => panic!("Unsupported sample rate. Expect 8 kHz or 16 kHz."),
    };
    if wav_reader.spec().sample_format != hound::SampleFormat::Int {
        panic!("Unsupported sample format. Expect Int.");
    }
    let content = wav_reader
        .samples()
        .filter_map(|x| x.ok())
        .collect::<Vec<i16>>();
    assert!(!content.is_empty());
    let silero = silero::Silero::new(sample_rate, "").unwrap();
    let vad_params = utils::VadParams {
        sample_rate: sample_rate.into(),
        ..Default::default()
    };
    let mut vad_iterator = vad_iter::VadIter::new(silero, vad_params);
    let mut last_state = speech_state::SpeechState::Silent;

    let sample_rate = 16000;

    for (i, audio_frame) in content.chunks_exact(vad_iterator.params.frame_size_samples).enumerate() {
        // 计算当前音频时间 (秒)
        let current_audio_time = i as f32* 0.032;
        
        match vad_iterator.process(audio_frame) {
            Ok(state) => {
                if state == speech_state::SpeechState::Speaking && last_state != state {
                    println!("开始说话 - 状态: {:?}, 音频时间: {:.3}秒", state, current_audio_time);
                } else if state == speech_state::SpeechState::StopSpeaking && last_state != state {
                    println!("结束说话 - 状态: {:?}, 音频时间: {:.3}秒", state, current_audio_time);
                }
                last_state = state;
            },
            Err(e) => println!("错误: {}", e),
        }
    }

    let total_audio_duration = content.len() as f32 / sample_rate as f32;
    println!("处理完成。音频总时长: {:.3}秒", total_audio_duration);
}