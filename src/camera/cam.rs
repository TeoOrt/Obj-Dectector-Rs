use anyhow::Result;
use opencv::{highgui, prelude::*};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use opencv::core::Scalar;
use crate::camera::cam_dection::ImageProcessor;

use super::cam_dection::VidObjDectector;
use super::cam_handler::CameraHandler;

#[derive(Debug, Clone)]
pub struct CameraFrame {
    pub timestamp: Instant,
    pub mat: Mat,
}

pub fn frames_controller(
    rx: Receiver<CameraFrame>,
    frame_holder: Arc<Mutex<Vec<CameraFrame>>>,
    a_key: Arc<Mutex<i32>>,
    obj_dectector : Arc<Mutex<VidObjDectector>>
) -> Result<()>{
    loop {
        let rcv_msg = rx.recv().unwrap();
        process_frame(&rcv_msg, frame_holder.clone(),obj_dectector.clone())?;
        if *a_key.lock().unwrap() == 'q' as i32 {
            break;
        }
    }
    Ok(())
}
pub fn process_frame(
    frame: &CameraFrame,
    frame_holder: Arc<Mutex<Vec<CameraFrame>>>,
    obj_dectector : Arc<Mutex<VidObjDectector>>
) -> Result<()> {
    let mut frame_lock = frame_holder.lock().unwrap();
    frame_lock.push(frame.clone());
    if frame_lock.len() > 10{
        let _ : Vec<Result<()>> = frame_lock.iter().map(|cam| {
            let obj_dt  = obj_dectector.clone();
            let mut lock_obj = obj_dt.lock().unwrap();
            lock_obj.infer_with_model(cam)
        }).collect();
        frame_lock.clear();
        println!("Flushing camera");
    }
    Ok(())
}

/// Function loops until letter q is pressed
/// Recommended to spawn thread
pub fn operate_cameras(
    cam: &CameraHandler,
    a_key: Arc<Mutex<i32>>,
    tx: Sender<CameraFrame>,
) -> Result<()> {
    // let mut frame = Mat::default();
    let mut frame = Mat::new_rows_cols_with_default(640, 640, opencv::core::CV_8UC3, Scalar::all(0.0))?;
    let mut big_struct = cam.inner.lock().unwrap();
    let camera_name = big_struct.display_window.clone().unwrap();
    let video_cap = &mut big_struct.camera;
    loop {
        video_cap.read(&mut frame)?;
        if frame.empty() {
            continue;
        }
        // Display the captured frame
        // making it debug
        highgui::imshow(&camera_name, &frame)?;
        let framer = CameraFrame {
            timestamp: Instant::now(),
            mat: frame.clone(),
        };

        if tx.send(framer).is_err() {
            break;
        }
        // Press 'q' to quit
        let mut key = a_key.lock().unwrap();
        *key = highgui::wait_key(10)?;
        if *key == 'q' as i32 {
            break;
        }
    }
    Ok(())
}
