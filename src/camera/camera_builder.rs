use crate::DisplayWindow;

use super::camera::Camera;
use anyhow::{Context, Result};
use opencv::highgui;
use opencv::videoio::{CAP_ANY, VideoCapture};
use opencv::{prelude::*, videoio};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct CameraBuilder {
    video_idx: Option<i32>,
    display_window: Option<Box<str>>,
    quit_key: Option<char>,
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
    pub fn with_quit_key(mut self, char_c: char) -> Self {
        self.quit_key = Some(char_c);
        self
    }
}

/// Constructor
impl CameraBuilder {
    pub fn build(self) -> Result<Camera> {
        let def_key = DisplayWindow::default().quit_key;
        let display_window = DisplayWindow {
            name: self.display_window.unwrap_or_default(),
            quit_key: self.quit_key.unwrap_or_else(||def_key)
        };

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
            display_window,
            camera,
        };
        Ok(Camera {
            inner: Arc::new(Mutex::new(inner)),
        })
    }
}


