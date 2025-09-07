use anyhow::Result;
use opencv::{
    core::Mat,
    videoio::{VideoCapture, VideoCaptureTrait},
};
use std::sync::{Arc, Mutex};

use crate::{ChannelID, EventServer, Message};

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
    pub event_server: Arc<EventServer>,
    pub mat: Box<Mat>,
}

#[derive(Clone)]
pub struct Camera {
    pub inner: Arc<Mutex<CameraInner>>,
}

impl CameraInner {
    pub fn get_frame(&mut self) -> Result<Mat> {
        self.camera.read(&mut (*self.mat))?;
        // sending back to display
        self.event_server.send(
            &ChannelID::WindowDisplay,
            Message::Frame(*(self.mat.clone())),
        );
        // Ok(frame)
        Ok(*(self.mat.clone()))
    }
}
