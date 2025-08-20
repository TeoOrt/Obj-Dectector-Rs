use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use camera_merger::{CameraBuilder,CameraOperator, Start, Stop};

#[test]
fn open_camera(){
    let camera = CameraBuilder::default()
                                .with_video_idx(0)
                                .build();

    assert!(camera.is_ok());
    let camera = camera.unwrap();
    let camera_lock = camera.inner.lock().unwrap();
    let camera_uwrp = camera_lock.display_window.quit_key;

    assert_eq!(camera_uwrp,'q');
}

#[test]
fn operation_of_camera(){
    let camera = CameraBuilder::default()
                                .with_video_idx(0)
                                .with_display_window("Matthe").unwrap()
                                .build();

    assert!(camera.is_ok());
    let camera = camera.unwrap();
    let cam_list = vec![camera];
    let (tx,_) =  channel();
    let mut cam_op = CameraOperator::new(cam_list,tx);
    cam_op.initialze();
    thread::sleep(Duration::new(1, 0));
    cam_op.start();
    thread::sleep(Duration::new(10, 0));
    cam_op.stop();
}

