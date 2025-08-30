use crate::interpreter::support::SimpleFlan;
use crate::{HrtProfiler, Profile, RtSync, ThreadOperation};
use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::{Arc, Mutex, mpsc::Sender};
use std::thread::JoinHandle;
use tract_onnx::prelude::tvec;
use tract_onnx::prelude::{IntoTensor, Tensor};

#[derive(Default)]
pub struct TensorPredictor {
    sync: Arc<Mutex<ThreadOperation>>,
    op_tr: Vec<JoinHandle<()>>,
    tx: Option<Sender<()>>,
}

impl TensorPredictor {
    // pub fn interpret_message(&mut self, model : &Session<'static>, tensor : Tensor) -> Result<()>{
    pub fn interpret_message(&mut self, model: &SimpleFlan, tensor: Vec<Tensor>) -> Result<()> {
        let mut prof = HrtProfiler::default();
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
        println!("{:?}", prof.get_stats());
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
