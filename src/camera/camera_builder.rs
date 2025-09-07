use crate::EventServer;

use super::camera::Camera;
use anyhow::{Context, Result};
use opencv::highgui;
use opencv::videoio::{CAP_ANY, VideoCapture};
use opencv::{prelude::*, videoio};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct CameraBuilder {
    video_idx: Option<i32>,
    display_window: Option<Box<str>>,
    event_server: Arc<EventServer>,
}

/// Builder methods
impl CameraBuilder {
    pub fn with_video_idx(mut self, idx: i32) -> Self {
        self.video_idx = Some(idx);
        self
    }
    pub fn with_display_window<S: Into<Box<str>>>(mut self, name: S) -> Result<Self> {
        let name_str = String::from(name.into());
        highgui::named_window(&name_str, highgui::WINDOW_AUTOSIZE)?;
        self.display_window = Some(name_str.into_boxed_str());
        Ok(self)
    }
    pub fn with_event_server(mut self, event_server: Arc<EventServer>) -> Self {
        self.event_server = event_server;
        self
    }
}

/// Constructor
impl CameraBuilder {
    pub fn build(self) -> Result<Camera> {
        let camera = VideoCapture::new(self.video_idx.unwrap(), CAP_ANY).with_context(|| {
            format!(
                "Failed to open camera index {} check dev",
                self.video_idx.unwrap_or_default()
            )
        })?;

        if !videoio::VideoCapture::is_opened(&camera)? {
            panic!("Failed to open video camera")
        }

        let inner = super::CameraInner {
            event_server: self.event_server,
            camera,
            mat: Box::new(Mat::default()),
        };
        Ok(Camera {
            inner: Arc::new(Mutex::new(inner)),
        })
    }
}
