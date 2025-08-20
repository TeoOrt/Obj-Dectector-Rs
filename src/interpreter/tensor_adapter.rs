use anyhow::{Result,anyhow};
use std::simd::{num::SimdUint, Simd};
use crate::HrtProfiler;
use tract_onnx::prelude::Tensor;
use tract_onnx::{tract_core::ndarray::Array4};
use opencv::{
    core::{Mat, MatTraitConst, Size},
};

use opencv::imgproc;


#[derive(Default)]
pub struct MatConverter {
    resized: Mat,
    rgb: Mat,
    profiler: HrtProfiler,
    chw_data: Vec<f32>,
}



impl MatConverter {
    fn resize_tensor(&mut self, mat : &Mat) -> Result<()>{
        imgproc::resize(
            &mat,
            &mut self.resized,
            Size::new(640, 640),
            0.0,
            0.0,
            imgproc::INTER_LINEAR,
        )?;
        Ok(())
    }
    fn recolor_to_rgb(&mut self) -> Result<()>{
        imgproc::cvt_color(
            &self.resized,
            &mut self.rgb,
            imgproc::COLOR_BGR2RGB,
            0,
            opencv::core::AlgorithmHint::ALGO_HINT_APPROX)?;
        Ok(())
    }
    
    pub fn mat_to_tensor(&mut self, mat: &Mat) -> Result<Tensor> {
        self.resize_tensor(mat)?;
        self.recolor_to_rgb()?;
        let (h, w) = (self.rgb.rows() as usize, self.rgb.cols() as usize);
        let dimensions = h * w * 3;
        let data: *const u8 = self.rgb.data();
        let data_slices: &[u8] = unsafe { std::slice::from_raw_parts(data, dimensions) };

        for c in 0..3{
            for y in 0..w{
                for x in 0..h{
                    let idx = (y * 640 + x) * 3 + c;
                    self.chw_data.push(data_slices[idx] as f32 / 255.0);
                }
            }
        }

        let array = Array4::from_shape_vec((1, 3, h, w), self.chw_data.clone())?;
        self.chw_data.clear();
        Ok(array.into())
    }
}
