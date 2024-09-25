
pub static MODEL: &[u8] = include_bytes!("../../model/silero_vad.onnx"); 

pub fn get_model() -> &'static [u8] {
    &MODEL
}