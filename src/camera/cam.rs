use anyhow::Result;
use opencv::videoio::VideoCapture;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::mpsc::Sender;

use opencv::{highgui, prelude::* };

#[derive(Debug, Clone)]
pub struct CameraFrame {
    pub timestamp: SystemTime,
    pub mat: Mat,
}

pub fn process_frame(
    frame: &CameraFrame,
    frame_holder: Arc<Mutex<Vec<CameraFrame>>>,
) -> Result<()> {
    let mut frame_lock = frame_holder.lock().unwrap();
    frame_lock.push(frame.clone());
    if frame_lock.len() > 59 {
        frame_lock.clear();
        println!("Flushing camera");
    }
    Ok(())
}


pub fn operate_cameras(cam : &mut VideoCapture, key : Arc<Mutex<i32>> , tx : Sender<CameraFrame>, cam_name : String) -> Result<()>
{
    let mut frame = Mat::default();
    loop {
        cam.read(&mut frame)?;
        if frame.empty() {
            continue; 
        }
        // Display the captured frame
        highgui::imshow(cam_name.as_str(), &frame)?;
        let framer = CameraFrame {
            timestamp: SystemTime::now(),
            mat: frame.clone(),
        };

        if tx.send(framer).is_err()
        {
            break;
        }


        // Press 'q' to quit
        let mut k = key.lock().unwrap();
        *k = highgui::wait_key(10)?;
        if *k == 'q' as i32 {
            break;
        }
        // thread::sleep(Duration::new(0, 10000));
    }
    Ok(())
}

// #[derive(Debug)]
// struct FullPicture
// {
//     timestamp : u64,
//     cam1 : CameraData,
//     cam2 : CameraData
// }
//
// fn combine_cameras(cam :&mut Vec<CameraData> , cam2 : &mut Vec<CameraData>, threshold : u64) -> Vec<FullPicture> {
//     cam.iter().zip(cam2).map(| cm1 |{
//         let (c1 , c2) = cm1;
//         let time_diff = c2.timestamp - c1.timestamp;
//         if time_diff > threshold {
//             c2.timestamp = c2.timestamp - threshold;
//         }
//         FullPicture{ timestamp: (c1.timestamp) , cam1:c1.clone(), cam2: c2.clone() }
//     }).collect()
// }
//
