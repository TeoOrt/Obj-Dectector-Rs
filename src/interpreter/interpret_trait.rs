use anyhow::Result;

struct Tensor{
    temp : u32
}


pub trait ImageProcessor {
    fn infer_with_model(&mut self, cam: Tensor ) -> Result<Tensor>;
}



