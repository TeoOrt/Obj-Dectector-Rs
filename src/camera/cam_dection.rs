use anyhow::Result;
use opencv::{
    core::{Mat, MatTraitConst},
    imgproc::{self},
};
use std::path::Path;
use tract_onnx::{prelude::*, tract_core::ndarray::Array4};
use crate::profilers::profile_structs::{HrtProfiler, Profile};
use super::cam::CameraFrame;

pub trait ImageProcessor {
    fn infer_with_model(&mut self, cam: &CameraFrame) -> Result<()>;
}


pub struct VidObjDectector {
    model: SimplePlan<
        TypedFact,
        Box<dyn TypedOp + 'static>,
        Graph<TypedFact, Box<dyn TypedOp + 'static>>,
    >,
    rgb :    Box<Mat>,
    profiler : HrtProfiler,
    chw_data : Box<Vec<f32>>
}

/// Basically a wrapper function for VidObjDectector
impl VidObjDectector {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            return Err(anyhow::anyhow!(
                "Configuration file not found: {}",
                path_ref.display()
            ));
        }

        let model = tract_onnx::onnx()
            .model_for_path(path)?
            .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec![1,3,640,640]))?
            .into_optimized()?
            .into_runnable()?;

        let mut chw_data= Box::new(Vec::new());
        chw_data.reserve(640*640*3);

        Ok(Self { model , profiler : HrtProfiler::new(), rgb: Box::new(Mat::default()), chw_data})
    }

    pub fn close(&self){
        let stats = &self.profiler.get_stats();
        println!("{:?}",stats);
    }


    /// Converts mat struct to tensor
    fn mat_to_tensor(&self, mat: &CameraFrame) -> Result<Tensor> {

    let mut rgb = *self.rgb.clone();
    imgproc::cvt_color(&mat.mat, &mut rgb, imgproc::COLOR_BGR2RGB, 0,opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT)?;

    let (h, w) = (rgb.rows() as usize, rgb.cols() as usize);
    let data: *const u8 = rgb.data();
    let data_slices: &[u8] = unsafe {
        std::slice::from_raw_parts(data, 3 * h * w)
    };

    let mut chw_data = *self.chw_data.clone();
    for c in 0..3 {
        for y in 0..h {
            for x in 0..w {
                let idx = (y * w + x) * 3 + c;
                chw_data.push(data_slices[idx] as f32 / 255.0);
            }
        }
    }

    // Create ndarray in shape (1, 3, 640, 640)
    let array = Array4::from_shape_vec((1, 3, h, w), chw_data)?;
    Ok(array.into())
    }
}

impl ImageProcessor for VidObjDectector {
    fn infer_with_model(&mut self, cam: &CameraFrame) -> Result<()> {
        self.profiler.start();
        let input = self.mat_to_tensor(cam)?;
        let _ = self.profiler.stop_and_record();

        //might have to mode this out to python
        let _ = self.model.run(tvec!(input.into()))?;
        println!("We ran model");
        Ok(())
    }
}
