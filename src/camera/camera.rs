use anyhow::Result;
use opencv::{
    core::Mat,
    videoio::{VideoCapture, VideoCaptureTrait},
};
use std::sync::{Arc, Mutex, mpsc::Sender};

#[derive(Debug)]
pub struct DisplayWindow {
    pub name: Box<str>,
    pub quit_key: char,
}
// ---> Default
impl Default for DisplayWindow {
    fn default() -> Self {
        Self {
            name: Box::from("Camera"),
            quit_key: 'q',
        }
    }
}

pub struct CameraInner {
    pub camera: VideoCapture,
    pub display_window: DisplayWindow,
    pub tx :Sender<Mat>
}

#[derive(Clone)]
pub struct Camera {
    pub inner: Arc<Mutex<CameraInner>>,
}

impl CameraInner {
    pub fn get_frame(&mut self) -> Result<Mat> {
        let mut frame = Mat::default();
        self.camera.read(&mut frame)?;
        self.tx.send(frame.clone())?;
        Ok(frame.clone())
    }
}

/// Getters
impl CameraInner {
    pub fn get_display_name(self) -> String {
        String::from(self.display_window.name)
    }
}
