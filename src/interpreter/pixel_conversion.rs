use tract_onnx::prelude::tract_data::half;

pub trait FromPixel: Sized {
    fn from_pixel(p: u8) -> Self;
}

impl FromPixel for f32 {
    #[inline]
    fn from_pixel(p: u8) -> Self {
        p as f32 / 255.0
    }
}

impl FromPixel for half::f16 {
    #[inline]
    fn from_pixel(p: u8) -> Self {
        half::f16::from_f32(p as f32 / 255.0)
    }
}

impl FromPixel for u8 {
    #[inline]
    fn from_pixel(p: u8) -> Self {
        p
    }
}
