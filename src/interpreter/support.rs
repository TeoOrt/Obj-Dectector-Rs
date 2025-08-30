use anyhow::Result;
use once_cell::sync::Lazy;
use onnxruntime::{GraphOptimizationLevel, environment::Environment, session::SessionBuilder};
use std::path::Path;
use tract_onnx::prelude::*;

use onnxruntime::session::Session;

pub type SimpleFlan =
    SimplePlan<TypedFact, Box<dyn TypedOp + 'static>, Graph<TypedFact, Box<dyn TypedOp + 'static>>>;

static ENV: Lazy<Environment> =
    Lazy::new(|| Environment::builder().with_name("Cpu_inf").build().unwrap());

pub fn open_onnx_model<P: AsRef<Path>>(path: P) -> Result<SimpleFlan> {
    let path_ref = path.as_ref();

    if !path_ref.exists() {
        return Err(anyhow::anyhow!(
            "Configuration file not found: {}",
            path_ref.display()
        ));
    }
    let model = tract_onnx::onnx()
        .model_for_path(path)?
        .with_input_fact(0, 
            InferenceFact::dt_shape(f32::datum_type(), tvec!(1,3,640,640))
            )?
        .into_optimized()?
        .into_runnable()?;
    // let sesh = ENV.new_session_builder()?.with_optimization_level(GraphOptimizationLevel::All)?.with_number_threads(8)?.with_model_from_file("yolov5s.onnx")?;
    // Ok(sesh)
    Ok(model)
}

pub fn open_onnx_runtime_session() -> Result<Session<'static>> {
    let sesh = ENV
        .new_session_builder()?
        .with_optimization_level(GraphOptimizationLevel::Extended)?
        .with_number_threads(8)?
        .with_model_from_file("yolov5s.onnx")?;
    Ok(sesh)
}
