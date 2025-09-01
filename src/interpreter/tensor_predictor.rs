use crate::interpreter::support::SimpleFlan;
use crate::{HrtProfiler, Profile, RtSync};
use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tract_onnx::prelude::tvec;
use tract_onnx::prelude::{IntoTensor, Tensor};
use std::collections::HashMap;

#[derive(Default)]
pub struct TensorPredictor {
    // sync: Arc<Mutex<ThreadOperation>>,
    // op_tr: Vec<JoinHandle<()>>,
    // tx: Option<Sender<()>>,
    profilers : HashMap<String,HrtProfiler>
}

impl TensorPredictor {
    fn get_profiler(&mut self, name:&str)-> &mut HrtProfiler{
        self.profilers.entry(name.to_string()).or_insert_with(HrtProfiler::default)
    }
    pub fn interpret_message(&mut self, model: &SimpleFlan, tensor: Vec<Tensor>) -> Result<()> {
        let prof = self.get_profiler("MsgInterpreter");
        prof.start();
        // Feeding a vec of 16 frames
        let _: Vec<_> = tensor
            .into_par_iter()
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

/// Rt capabilites
impl RtSync for TensorPredictor {
    fn start(&mut self) {
        todo!()
    }
    fn stop(&mut self) {
        todo!()
    }
}
