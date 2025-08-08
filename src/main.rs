use anyhow::Result;
use camera_merger::camera::cam::{operate_cameras, process_frame, CameraFrame};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self}, time::Duration,
};

use opencv::{highgui, prelude::*, videoio};

fn main() -> Result<()> {
    // Open default camera (index 0)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if !videoio::VideoCapture::is_opened(&cam)?{
        return Err(anyhow::anyhow!("Unable to open default first camera!"));
    }
    let mut cam2 = videoio::VideoCapture::new(3, videoio::CAP_ANY)?;
    if !videoio::VideoCapture::is_opened(&cam2)?{
        return Err(anyhow::anyhow!("Unable to open default camera!"));
    }

    // Create a window to show camera feed
    highgui::named_window("Camera", highgui::WINDOW_AUTOSIZE)?;
    highgui::named_window("Camera2", highgui::WINDOW_AUTOSIZE)?;

    let key: Arc<Mutex<i32>> = Arc::new(Mutex::new(10));
    let frames_lock: Arc<Mutex<Vec<CameraFrame>>> = Arc::new(Mutex::new(Vec::new()));
    let (tx, rx): (mpsc::Sender<CameraFrame>, mpsc::Receiver<CameraFrame>) = mpsc::channel();
    frames_lock.lock().unwrap().reserve(60);

    //thread copies
    let hanlde_key = key.clone();
    let handle_frames = frames_lock.clone();
    let handle = thread::spawn(move || {
        loop {
            let rcv_msg = rx.recv().unwrap();
            process_frame(&rcv_msg, handle_frames.clone()).unwrap();
            if *hanlde_key.lock().unwrap() == 'q' as i32 {
                break;
            }
        }
    });
    let key_hd = key.clone();
    let tx_d = tx.clone();
    let key_hd2 = key.clone();
    let tx_d2 = tx.clone();

    let handle_1 = thread::spawn(move||{
        let _ = operate_cameras(&mut cam, key_hd, tx_d, String::from("Camera2")).unwrap();
    });
    let hanlde_2 = thread::spawn(move || {
        let _ = operate_cameras(&mut cam2, key_hd2.clone(), tx_d2.clone(), String::from("Camera")).unwrap();
    });

    handle.join().unwrap();
    handle_1.join().unwrap();
    hanlde_2.join().unwrap();

    Ok(())
}
