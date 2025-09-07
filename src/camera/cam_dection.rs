use super::cam::CameraFrame;
use tract_onnx::prelude::Tensor;

use crate::profilers::profile_structs::{HrtProfiler, Profile};
use anyhow::{Result, anyhow};
use opencv::{
    core::{Mat, MatTraitConst, Size},
    imgproc,
};
use serde::Deserialize;
use std::simd::{Simd, num::SimdUint};
use std::{collections::BTreeMap, fs::File, path::Path};
use tract_onnx::{prelude::*, tract_core::ndarray::Array4};

static LENGHT: usize = 640;
static WIDTH: usize = 640;
static DIMENSIONS: usize = 3;
static IMAGE_DIM: usize = LENGHT * WIDTH * DIMENSIONS;

pub struct VidObjDectector {
    pub labels: BTreeMap<u32, String>,
    model: SimplePlan<
        TypedFact,
        Box<dyn TypedOp + 'static>,
        Graph<TypedFact, Box<dyn TypedOp + 'static>>,
    >,
    resized: Box<Mat>,
    rgb: Box<Mat>,
    profiler: HrtProfiler,
    chw_data: Box<Vec<f32>>,
}

#[derive(Debug, Deserialize)]
struct CocoNames {
    names: BTreeMap<u32, String>,
}

/// Basically a wrapper function for VidObjDectector
impl VidObjDectector {
    pub fn new<P: AsRef<Path>>(path: P, labels: P) -> Result<Self> {
        let path_ref = path.as_ref();
        let labels_ref = labels.as_ref(); // yaml file

        if !path_ref.exists() || !labels_ref.exists() {
            return Err(anyhow::anyhow!(
                "Configuration file not found: {}",
                path_ref.display()
            ));
        }

        let file = File::open(labels_ref)?;
        let labels: CocoNames = serde_yaml::from_reader(file)?;

        let model = tract_onnx::onnx()
            .model_for_path(path)?
            .with_input_fact(
                0,
                InferenceFact::dt_shape(f32::datum_type(), tvec![1, 3, LENGHT, WIDTH]),
            )?
            .into_optimized()?
            .into_runnable()?;

        let mut chw_data = Box::new(vec![0f32; DIMENSIONS * LENGHT * WIDTH]);
        chw_data.reserve(IMAGE_DIM);

        Ok(Self {
            model,
            profiler: HrtProfiler::new(),
            resized: Box::new(Mat::default()),
            rgb: Box::new(Mat::default()),
            chw_data,
            labels: labels.names,
        })
    }

    pub fn close(&self) {
        let stats = &self.profiler.get_stats();
        println!("{:?}", stats);
    }

    /// Converts mat struct to tensor
    fn mat_to_tensor(&mut self, mat: &CameraFrame) -> Result<Tensor> {
        //looks like we need some resizing
        let mut resized = *self.resized.clone();
        let mut rgb = *self.rgb.clone();

        imgproc::resize(
            &mat.mat,
            &mut resized,
            Size::new(640, 640),
            0.0,
            0.0,
            imgproc::INTER_LINEAR,
        )?;

        match imgproc::cvt_color(
            &resized,
            &mut rgb,
            imgproc::COLOR_BGR2RGB,
            0,
            opencv::core::AlgorithmHint::ALGO_HINT_APPROX,
        ) {
            Ok(()) => {}
            Err(err) => {
                return Err(anyhow!(
                    "We got error while processing image to rgb {:?}",
                    err
                ));
            }
        };

        let (h, w) = (rgb.rows() as usize, rgb.cols() as usize);

        let data: *const u8 = rgb.data();
        let data_slices: &[u8] = unsafe { std::slice::from_raw_parts(data, IMAGE_DIM) };

        self.profiler.start();
        let mut chw_data = *self.chw_data.clone();
        let scale = 1.0 / 255.0;

        let sscale = Simd::splat(scale);
        let chunks = data_slices.chunks_exact(8);
        // let remainder = chunks.remainder();

        for (chunk, out_chunk) in chunks.zip(chw_data.chunks_exact_mut(8)) {
            let vals = Simd::<u8, 8>::from_slice(chunk);
            let vals_f32 = vals.cast::<f32>() * sscale;
            vals_f32.copy_to_slice(out_chunk);
        }

        let _ = self.profiler.stop_and_record();

        // Create ndarray in shape (1, 3, 640, 640)
        let array = Array4::from_shape_vec((1, 3, h, w), chw_data)?;
        Ok(array.into())
    }
}

// impl ImageProcessor for VidObjDectector {
//     fn infer_with_model(&mut self, cam: &CameraFrame) -> Result<Tensor> {
//         let input = match self.mat_to_tensor(cam) {
//             Ok(success) => success,
//             Err(e) => {
//                 return Err(anyhow::anyhow!(
//                     "Error was detected during processing of frame to tensor {:?}",
//                     e
//                 ));
//             }
//         };
//         let result_tensor = self.model.run(tvec!(input.into()))?;
//         let ve: &Tensor = &result_tensor[0];
//         let output_array = ve.to_array_view::<f32>().expect("Nope");
//         let ( _, num_preds, num_attrs) = output_array.dim();
//         for pred in output_array.index_axis(ndarray::Axis(0), 0).outer_iter(){
//         }
//
//
//
//         todo!()
//         // Ok(result_tensor.into())
//     }
// }
