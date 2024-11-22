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

    for audio_frame in content.chunks_exact(vad_iterator.params.frame_size_samples) {
        let res = vad_iterator.process(audio_frame);
        match res {
            Ok(state) => {
                println!("Current state is {:?}", state);
            },
            Err(e) => println!("Error: {}", e),
        }
    }

    // let res = vad_iterator.process(&content);
    // if res.is_err() {
    //     println!("Error: {}", res.err().unwrap());
    // }

    println!("Finished.");
}