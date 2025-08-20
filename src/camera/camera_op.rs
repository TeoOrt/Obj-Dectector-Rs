use crate::{wait_for_start_or_stop, Camera, Start, Stop};
use crate::ThreadOperation;
use anyhow::Result;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::{
    thread::{self, JoinHandle},
};

use opencv::prelude::*;


pub struct CameraOperator {
    cameras: Vec<Camera>,
    thread_handles: Vec<JoinHandle<()>>,
    operation: Arc<Mutex<ThreadOperation>>,
    tx       : Sender<Mat>
}

/// Helper functions
impl CameraOperator {
    fn start_loop(operation: Arc<Mutex<ThreadOperation>>,camera : Camera, tx : Sender<Mat>) ->Result<()> {
        loop{
            match *operation.lock().unwrap() {
                ThreadOperation::STOP => break,
                _ => (),
            };
            let mut cam = camera.inner.lock().unwrap();
            let frame = &cam.get_frame()?;
            cam.display_video(frame)?;
            tx.send(frame.clone())?;
        }
        Ok(())
    }
}

/// Setters
impl CameraOperator {
    pub fn append_cameras(&mut self, camera_list: Vec<Camera>) -> &Self {
        for cameras in camera_list {
            self.cameras.push(cameras.clone());
        }
        self
    }
    fn process_camera_frames(&mut self) -> Result<()> {
        let tx = self.tx.clone();
        self.thread_handles = self
            .cameras
            .iter()
            .map(|camera| {
                let cam_clone = camera.clone();
                let op_clone = self.operation.clone();
                let tx_clone = tx.clone();
                thread::spawn(move || {
                    wait_for_start_or_stop(op_clone.clone());
                    CameraOperator::start_loop(op_clone,cam_clone,tx_clone).unwrap();
                })
            })
            .collect();
        Ok(())
    }
}

/// Sync Traits
impl Start for CameraOperator {
    fn start(&mut self) {
        *self.operation.lock().unwrap() = ThreadOperation::START;
    }
}

impl Stop for CameraOperator {
    fn stop(&mut self) {
        *self.operation.lock().unwrap() = ThreadOperation::STOP;
        for th in self.thread_handles.drain(..) {
            th.join().unwrap();
        }
    }
}

/// Constructors
impl CameraOperator {
    pub fn new(camera_list: Vec<Camera>, tx : Sender<Mat>) -> Self {
        Self { cameras: camera_list, thread_handles: Vec::new(), operation: Arc::new(Mutex::new(ThreadOperation::default())), tx }
    }
    pub fn initialze(&mut self) -> &Self {
        self.process_camera_frames().unwrap();
        self
    }
}
