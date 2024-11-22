use crate::ort_vad::utils;
use anyhow::anyhow;
use ndarray::{s, Array, Array2, ArrayBase, ArrayD, Dim, IxDynImpl, OwnedRepr};
use std::error::Error;
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct Silero {
    session: ort::Session,
    sample_rate: ArrayBase<OwnedRepr<i64>, Dim<[usize; 1]>>,
    state: ArrayBase<OwnedRepr<f32>, Dim<IxDynImpl>>,
}


// only support the 32ms frame size
impl Silero {
    // there should be two new function once for macos/ios and one for linux/android
    // the function should be named new_for_macos and new_for_linux
    // use target_os to determine the platform
    // for macos/ios, the function should load the model from the bundle
    // for linux/android, the function should load the model from the asset manager
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub fn new(
        sample_rate: utils::SampleRate,
        model_path: impl AsRef<Path>,
    ) -> Result<Self, ort::Error> {
        let session = if model_path.as_ref().exists() {
            ort::Session::builder()?.commit_from_file(model_path)?
        } else {
            // If the file doesn't exist, use the bytes from model.rs
            let providers = vec![ort::CoreMLExecutionProvider::default().with_subgraphs().with_ane_only().build()];
            let model_bytes = crate::exposure::model::get_model();
            ort::Session::builder()?
            .with_execution_providers(providers)?
            .commit_from_memory(model_bytes)?
        };
        // currently no support for batch
        const BATCH: usize = 1;
        let state = ArrayD::<f32>::zeros([2, BATCH, 128].as_slice());
        let sample_rate = Array::from_shape_vec([1], vec![sample_rate.into()]).unwrap();
        Ok(Self {
            session,
            sample_rate,
            state,
        })
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn new(
        sample_rate: utils::SampleRate,
        model_path: impl AsRef<Path>,
    ) -> Result<Self, ort::Error> {
        let session = if model_path.as_ref().exists() {
            ort::Session::builder()?.commit_from_file(model_path)?
        } else {
            let providers = vec![ort::ExecutionProviderDispatch::CpuExecutionProvider(ort::NNAPIExecutionProvider::default().build())];
            let model_bytes = crate::exposure::model::get_model();
            ort::Session::builder()?
            .with_execution_providers(providers)?
            .commit_from_memory(model_bytes)?
        };
        let state = ArrayD::<f32>::zeros([2, 1, 128].as_slice());
        let sample_rate = Array::from_shape_vec([1], vec![sample_rate.into()]).unwrap();
        Ok(Self {
            session,
            sample_rate,
            state,
        })
    }
    

    pub fn reset(&mut self) {
        self.state = ArrayD::<f32>::zeros([2, 1, 128].as_slice());
    }

    pub fn calc_level(&mut self, audio_frame: &[i16]) -> Result<f32, anyhow::Error> {
        if audio_frame.len() != 512 {
            return Err(anyhow!("Invalid frame size"));
        }

        let data = audio_frame
            .iter()
            .map(|x| (*x as f32) / (i16::MAX as f32))
            .collect::<Vec<_>>();
        let mut frame = Array2::<f32>::from_shape_vec([1, data.len()], data).unwrap();
        frame = frame.slice(s![.., ..480]).to_owned();
        let inps = ort::inputs![
            frame,
            std::mem::take(&mut self.state),
            self.sample_rate.clone(),
        ]?;
        let res = self
            .session
            .run(ort::SessionInputs::ValueSlice::<3>(&inps))?;
        self.state = res["stateN"].try_extract_tensor()?.to_owned();
        Ok(*res["output"]
            .try_extract_raw_tensor::<f32>()
            ?
            .1
            .first()
            .unwrap())
    }
}
