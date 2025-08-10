use std::ffi::CStr;
use std::path::Path;

use anyhow::Result;
use onnxruntime::ndarray::{Array, IxDyn};
use onnxruntime::tensor::OrtOwnedTensor;
use onnxruntime::{environment::Environment, session::Session};
use opencv::core::{CV_32F, Mat, MatTraitConst, Size};
use opencv::flann::CS;
use opencv::imgproc::{self, COLOR_BGR2RGB};

use super::cam::CameraFrame;

pub struct VidObjDectector<'a> {
    model: Session<'a>,
}

// Definetly want to add profiler to this function not sure how good it is
pub fn pre_process_frame(frame: &Mat) -> Result<Vec<f32>> {
    let mut resized = Mat::default();
    imgproc::resize(
        &frame,
        &mut resized,
        Size::new(640, 640),
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )?;

    let mut rgb = Mat::default();
    imgproc::cvt_color(
        &resized,
        &mut rgb,
        COLOR_BGR2RGB,
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT,
    )?;

    let mut rgb_f32 = Mat::default();
    rgb.convert_to(&mut rgb_f32, CV_32F, 1.0 / 255.0, 0.0)?;

    let total = (rgb_f32.rows() * rgb_f32.cols() * rgb_f32.channels()) as usize;
    let data: Vec<f32> =
        unsafe { std::slice::from_raw_parts(rgb_f32.ptr(0)? as *const f32, total).to_vec() };
    // Convert from HWC to CHW
    let (h, w, c) = (
        rgb_f32.rows() as usize,
        rgb_f32.cols() as usize,
        rgb_f32.channels() as usize,
    );
    let mut chw_data = Vec::with_capacity(total);
    for ch in 0..c {
        for row in 0..h {
            for col in 0..w {
                let index = row * w * c + col * c + ch;
                chw_data.push(data[index]);
            }
        }
    }
    Ok(chw_data)
}

impl<'a> VidObjDectector<'a> {
    pub fn new(environment: &'a Environment) -> Result<Self> {
        let path_ref = Path::new("yolov5s.onnx");
        if !path_ref.exists() {
            return Err(anyhow::anyhow!(
                "Configuration file not found: {}",
                path_ref.display()
            ));
        }
        let model = environment
            .new_session_builder()?
            .with_optimization_level(onnxruntime::GraphOptimizationLevel::Basic)?
            .with_number_threads(4)?
            .with_model_from_file("yolov5s.onnx")?;
        Ok(Self { model })
    }
    pub fn infer_with_model(&mut self, frame: &CameraFrame) -> Result<()> {
        //profile this space
        let input_tensor = pre_process_frame(&frame.mat)?;
        let input_array = Array::from_shape_vec((1, 3, 640, 640), input_tensor)?;

        // let input_names = self.model.inputs[0].name.clone();
        let inputs = vec![input_array];
        let outputs : Vec<OrtOwnedTensor<f32,_>> = self
            .model
            .run(inputs)?;

        // getting output
        let out_tens = &outputs[0];
        let shape = out_tens.shape();

        println!("Ouput shape: {:?}",shape);

        todo!()
    }
}
