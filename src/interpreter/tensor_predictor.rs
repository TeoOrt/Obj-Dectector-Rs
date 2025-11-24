use crate::interpreter::support::SimpleFlan;
use crate::labels::{self, Labels, get_labels};
use crate::{HrtProfiler, MatConverter, Profile, open_onnx_model};
use anyhow::Result;
use onnxruntime::ndarray::{self, ArrayView2, Axis};
use opencv::core::Mat;
use opencv::{highgui, imgcodecs};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::BTreeMap;
use std::time::Duration;
use std::{thread, u8};
use tract_onnx::prelude::tvec;
use tract_onnx::prelude::{IntoTensor, Tensor};
use tract_onnx::tract_core::ndarray::Ix3;

pub struct Dectection {
    pub label: Box<str>,
    pub confidence: f32,
    pub bbox: (i32, i32, i32, i32),
}

#[derive(Default)]
pub struct TensorPredictor {
    profilers: BTreeMap<String, HrtProfiler>,
}
fn decode_output(output: &ndarray::ArrayViewD<f32>) -> Vec<Dectection> {
    // let path = "Pylearn/data/coco.yaml";
    // let labels = get_labels(path).unwrap();
    // let mut detections = Vec::new();
    // for row in arr.outer_iter() {
    //     let conf = row[4];
    //     if conf > 0.25 {
    //         let (class_id, class_score) = row
    //             .iter()
    //             .skip(5)
    //             .enumerate()
    //             .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
    //             .unwrap();
    //         let final_conf = conf * class_score;
    //         if final_conf > 0.25 {
    //             let label = labels.names.get(&(class_id as i16)).unwrap();
    //             detections.push(Dectection {
    //                 label: label.clone().into_boxed_str(),
    //                 confidence: final_conf,
    //                 bbox: (row[0] as i32, row[1] as i32, row[2] as i32, row[3] as i32),
    //             });
    //         }
    //     }
    // }
    // detections
    todo!()
}
fn draw_detections(mut img: Mat, detections: &[Dectection]) -> opencv::Result<Mat> {
    use opencv::core::{Point, Scalar};
    use opencv::imgproc;
    for det in detections {
        let (x, y, w, h) = det.bbox;

        // Draw rectangle
        imgproc::rectangle(
            &mut img,
            opencv::core::Rect::new(x, y, w, h),
            Scalar::new(0.0, 255.0, 0.0, 0.0), // green box
            2,
            imgproc::LINE_8,
            0,
        )?;

        // Put label text
        let label = format!("{} {:.2}", det.label, det.confidence);
        imgproc::put_text(
            &mut img,
            &label,
            Point::new(x, y - 10),
            imgproc::FONT_HERSHEY_SIMPLEX,
            0.5,
            Scalar::new(0.0, 255.0, 0.0, 0.0),
            1,
            imgproc::LINE_8,
            false,
        )?;
    }
    Ok(img)
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
        let outputs: Vec<_> = tensor
            .par_iter()
            .filter_map(|tens| {
                model
                    .run(tvec!(tens.clone().into()))
                    .map(|mut res| res.remove(0).into_tensor())
                    .ok()
            })
            .collect();
        eprintln!("Size of output is {}", outputs.len());
        // for output in outputs {
        //     let data = output.to_array_view::<f32>()?.into_dimensionality::<Ix3>();
        //     // let dectect = decode_output(data);
        //     let path = "Pylearn/data/coco.yaml";
        //     let labels = get_labels(path).unwrap();
        //     let mut detections = Vec::new();
        //     for row in data.iter {
        //         let conf = row[4];
        //         if conf > 0.25 {
        //             let (class_id, class_score) = row
        //                 .iter()
        //                 .skip(5)
        //                 .enumerate()
        //                 .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        //                 .unwrap();
        //             let final_conf = conf * class_score;
        //             if final_conf > 0.25 {
        //                 let label = labels.names.get(&(class_id as i16)).unwrap();
        //                 detections.push(Dectection {
        //                     label: label.clone().into_boxed_str(),
        //                     confidence: final_conf,
        //                     bbox: (row[0] as i32, row[1] as i32, row[2] as i32, row[3] as i32),
        //                 });
        //             }
        //         }
        //     }
        //     let img = imgcodecs::imread("download.jpeg", imgcodecs::IMREAD_COLOR).unwrap();
        //     let res = draw_detections(img, &detections)?;
        //     highgui::imshow("Dectections", &res).unwrap();
        //     highgui::wait_key(0).unwrap();
        //     thread::sleep(Duration::new(4, 0));
        // }
        // let img = imgcodecs::imread("download.jpeg", imgcodecs::IMREAD_COLOR).unwrap();
        // highgui::imshow("Dectections", &img).unwrap();
        // highgui::wait_key(0).unwrap();
        //
        // prof.stop_and_record();
        Ok(())
    }

    pub fn record_labels() {}
    //todo get predictions
    //then display back onto gui ( Optional )
    //then we are done
    //maybe test it with two cameras
    //and compare performance
}

#[test]
pub fn test_tensor_prediction() {
    let flan = open_onnx_model("yolov5s.onnx").unwrap();
    let mut predictor = TensorPredictor::default();
    let mut converter = MatConverter::default();
    let mut img_store = Vec::new();
    for _ in 0..2 {
        let img = imgcodecs::imread("download.jpeg", imgcodecs::IMREAD_COLOR).unwrap();
        img_store.push(img);
    }

    let _ =
        predictor.interpret_message(&flan, converter.mats_to_tensor::<f32>(&img_store).unwrap());

    assert_eq!(0, 0);
}
