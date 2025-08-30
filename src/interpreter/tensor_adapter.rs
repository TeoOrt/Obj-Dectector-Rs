use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
// use std::simd::{num::SimdUint, Simd};
use opencv::core::{Mat, MatTraitConst, Size};
use tract_onnx::prelude::Tensor;
use tract_onnx::tract_core::ndarray::Array4;

use opencv::imgproc;

#[derive(Default)]
pub struct MatConverter {
    resized: Mat,
    rgb: Mat,
}

impl MatConverter {
    fn resize_tensor(&mut self, mat: &Mat) -> Result<()> {
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
    fn recolor_to_rgb(&mut self) -> Result<()> {
        imgproc::cvt_color(
            &self.resized,
            &mut self.rgb,
            imgproc::COLOR_BGR2RGB,
            0,
            opencv::core::AlgorithmHint::ALGO_HINT_APPROX,
        )?;
        Ok(())
    }
    pub fn mats_to_tensor(&mut self, mats: &Vec<Mat>) -> Result<Vec<Tensor>> {
        let (h, w) = (640, 640); // pixel size
        let dimensions = h * w * 3;
        let mut chw_vec = Vec::new();
        let mut chw_data = vec![0.0; dimensions];
        for mat in mats.iter() {
            self.resize_tensor(mat)?;
            self.recolor_to_rgb()?;
            let data = self.rgb.data();
            let data_slices: &[u8] = unsafe { std::slice::from_raw_parts(data, h * w * 3) };
            for c in 0..3 {
                for y in 0..h {
                    for x in 0..w {
                        let hwc_idx = (y * w + x) * 3 + c;
                        let chw_idx = c * (h * w) + (y * w + x);
                        chw_data[chw_idx] = data_slices[hwc_idx] as f32 / 255.0;
                    }
                }
            }
            chw_vec.push(chw_data.clone());
        }

        let array: Vec<Tensor> = chw_vec
            .into_par_iter()
            .map(|chw| Array4::from_shape_vec((1, 3, h, w), chw).unwrap().into())
            .collect();

        Ok(array.into())
    }
}
