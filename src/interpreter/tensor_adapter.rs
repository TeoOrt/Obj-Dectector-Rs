use anyhow::Result;
// use std::simd::{num::SimdUint, Simd};
use opencv::core::{Mat, MatTraitConst, Size};
use tract_onnx::prelude::{Datum, Tensor};
use tract_onnx::tract_core::ndarray::Array4;

use opencv::imgproc;

use crate::interpreter::pixel_conversion::FromPixel;

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
    pub fn mats_to_tensor<P>(&mut self, mats: &Vec<Mat>) -> Result<Vec<Tensor>>
    where
        P: FromPixel + Clone + Default + Datum,
    {
        let (h, w) = (640, 640); // pixel size
        let dimensions = h * w * 3;
        let mut chw_vec = Vec::new();
        for mat in mats.iter() {
            self.resize_tensor(mat)?;
            self.recolor_to_rgb()?;
            let data = self.rgb.data();
            let data_slices: &[u8] = unsafe { std::slice::from_raw_parts(data, dimensions) };
            let mut chw_data = vec![P::default(); dimensions];
            for c in 0..3 {
                for y in 0..h {
                    for x in 0..w {
                        let hwc_idx = (y * w + x) * 3 + c;
                        let chw_idx = c * (h * w) + (y * w + x);
                        chw_data[chw_idx] = P::from_pixel(data_slices[hwc_idx]);
                    }
                }
            }
            chw_vec.push(chw_data);
        }
        let mut array = Vec::new();
        for arry in chw_vec {
            array.push(Array4::from_shape_vec((1, 3, h, w), arry)?.into());
        }

        Ok(array.into())
    }
}
