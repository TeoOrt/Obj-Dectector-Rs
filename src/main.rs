use anyhow::Result;
use camera_merger::camera::{
    cam::{CameraFrame, frames_controller, operate_cameras},
    cam_cfg::CameraConfig,
    cam_dection::VidObjDectector,
    cam_handler::{CameraBuilder, CameraHandler},
};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
    thread::{self},
};

struct Args {
    key: Arc<Mutex<i32>>,
    frames_lock: Sender<CameraFrame>,
    camera: CameraHandler,
}

fn main() -> Result<()> {
    // Create a window to show camera feed
    let cfg = CameraConfig::from_file("CamConfig.toml")?;
    let cameras: Vec<CameraHandler> = cfg
        .get_video_device_list()?
        .iter()
        .map(|dev_idx| {
            CameraBuilder::new()
                .video_idx(dev_idx.clone())
                .display_window(format!("Camera{}", dev_idx))
                .build()
                .unwrap()
        })
        .collect();
    let obj_dectector = Arc::new(Mutex::new(VidObjDectector::new("yolov5s.onnx")?));
    let key: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let frames_lock: Arc<Mutex<Vec<CameraFrame>>> = Arc::new(Mutex::new(Vec::new()));
    let (tx, rx): (Sender<CameraFrame>, Receiver<CameraFrame>) = mpsc::channel();
    frames_lock.lock().unwrap().reserve(60);

    let clone_list: Vec<Args> = cameras
        .iter()
        .map(|cam| Args {
            key: key.clone(),
            frames_lock: tx.clone(),
            camera: cam.clone(),
        })
        .collect();

    let thread_handles: Vec<JoinHandle<()>> = clone_list
        .into_iter()
        .map(|args| {
            thread::spawn(move || {
                let _ = operate_cameras(&args.camera.clone(), args.key, args.frames_lock).unwrap();
            })
        })
        .collect();

    // main thread
    frames_controller(rx, frames_lock.clone(), key.clone(), obj_dectector.clone())?;

    for handle in thread_handles {
        handle.join().unwrap();
    }
    obj_dectector.clone().lock().unwrap().close();
    Ok(())
}
