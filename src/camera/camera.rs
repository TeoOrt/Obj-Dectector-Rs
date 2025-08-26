use std::sync::{Arc, Mutex};
use opencv::{core::Mat, highgui, videoio::{VideoCapture, VideoCaptureTrait}};
use anyhow::Result;

#[derive(Debug)]
pub struct DisplayWindow{
    pub name: Box<str>,
    pub quit_key: char
}
// ---> Default
impl Default for DisplayWindow{
    fn default() -> Self {
        Self { name: Box::from("Camera"), quit_key: 'q'}
    }
}

pub struct CameraInner {
    pub camera: VideoCapture,
    pub display_window: DisplayWindow,
}

#[derive(Clone)]
pub struct Camera {
    pub inner: Arc<Mutex<CameraInner>>,
}

impl CameraInner{
    pub fn get_frame(&mut self)-> Result<Mat>{
        let mut frame = Mat::default();
        self.camera.read(&mut frame)?;
        Ok(frame.clone())
    }
    pub fn display_video(&mut self, frame : &Mat)->Result<char>{
        highgui::imshow(&self.display_window.name, frame)?;
        let key_pressed = highgui::wait_key(1)? as u32;
        Ok(char::from_u32(key_pressed.into()).unwrap_or_default())
    }
}

/// Getters
impl CameraInner{
    pub fn get_display_name(self)-> String{
        String::from(self.display_window.name)
    }

}

