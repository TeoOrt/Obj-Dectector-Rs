use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
// use std::simd::{num::SimdUint, Simd};
use tract_onnx::prelude::Tensor;
use tract_onnx::{tract_core::ndarray::Array4};
use opencv::{
    core::{Mat, MatTraitConst, Size},
};
use std::sync::{Arc,Mutex};
use std::time::Instant;

use opencv::imgproc;

use crate::{HrtProfiler, Profile};


#[derive(Default)]
pub struct MatConverter {
    resized: Mat,
    rgb: Mat,
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
        let mut prof = HrtProfiler::default();
        self.resize_tensor(mat)?;
        self.recolor_to_rgb()?;
        let (h, w) = (self.rgb.rows() as usize, self.rgb.cols() as usize);
        let dimensions = h * w * 3;
        let data: *const u8 = self.rgb.data();
        let data_slices: &[u8] = unsafe { std::slice::from_raw_parts(data, dimensions) };

        // for c in 0..3{
        //     for y in 0..w{
        //         for x in 0..h{
        //             let idx = (y * 640 + x) * 3 + c;
        //             self.chw_data.push(data_slices[idx] as f32 / 255.0);
        //         }
        //     }
        // }
        let mut chw_data = vec![0.0 ;dimensions];
        prof.start();
        (0..3).into_iter().for_each(|c|{
            for y in 0..w{
                for x in 0..w {
                    let hwc_idx = (y * w + x) * 3 + c;          // source: HWC
                    let chw_idx = c * (h * w) + (y * w + x);    // target: CHW
                    chw_data[chw_idx] = data_slices[hwc_idx] as f32 / 255.0;
            }
            }
        });
        prof.stop_and_record();
        println!("{:?}", prof.get_stats());
        let array = Array4::from_shape_vec((1, 3, h, w), chw_data)?;
        Ok(array.into())
    }
}
