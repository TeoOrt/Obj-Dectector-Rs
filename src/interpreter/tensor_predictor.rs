use crate::interpreter::support::SimpleFlan;
use crate::{HrtProfiler, Profile};
use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use tract_onnx::prelude::tvec;
use tract_onnx::prelude::{IntoTensor, Tensor};

#[derive(Default)]
pub struct TensorPredictor {
    profilers: HashMap<String, HrtProfiler>,
}

impl TensorPredictor {
    fn get_profiler(&mut self, name: &str) -> &mut HrtProfiler {
        self.profilers
            .entry(name.to_string())
            .or_insert_with(HrtProfiler::default)
    }
    pub fn interpret_message(&mut self, model: &SimpleFlan, tensor: Vec<Tensor>) -> Result<()> {
        let prof = self.get_profiler("MsgInterpreter");
        prof.start();
        // Feeding a vec of 16 frames
        let _: Vec<_> = tensor
            .par_iter()
            .filter_map(|tens| {
                model
                    .run(tvec!(tens.clone().into()))
                    .map(|mut res| res.remove(0).into_tensor())
                    .ok()
            })
            .collect();

        prof.stop_and_record();
        Ok(())
    }
}
