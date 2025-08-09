use anyhow::Result;
use opencv::{highgui, prelude::*};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Instant;

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
) {
    loop {
        let rcv_msg = rx.recv().unwrap();
        process_frame(&rcv_msg, frame_holder.clone()).unwrap();
        if *a_key.lock().unwrap() == 'q' as i32 {
            break;
        }
    }
}
pub fn process_frame(
    frame: &CameraFrame,
    frame_holder: Arc<Mutex<Vec<CameraFrame>>>,
) -> Result<()> {
    let mut frame_lock = frame_holder.lock().unwrap();
    frame_lock.push(frame.clone());
    if frame_lock.len() > 10 {
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
    let mut frame = Mat::default();
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
