use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use opencv::highgui;
use opencv::videoio::{CAP_ANY, VideoCapture};

pub struct CameraBuilder {
    video_idx: Option<i32>,
    display_window: Option<Box<str>>,
    quit_key: Option<i32>,
}

pub struct CameraHandlerInner {
    pub camera: VideoCapture,
    pub display_window: Option<Box<str>>,
    pub quit_key: Arc<Mutex<i32>>,
}

#[derive(Clone)]
pub struct CameraHandler {
    pub inner: Arc<Mutex<CameraHandlerInner>>,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            video_idx: None,
            display_window: None,
            quit_key: None,
        }
    }
    pub fn video_idx(mut self, idx: i32) -> Self {
        self.video_idx = Some(idx);
        self
    }
    pub fn display_window<S: Into<Box<str>>>(mut self, name: S) -> Self {
        self.display_window = Some(name.into());
        self
    }
    pub fn set_quit_key(mut self, char_c: char) -> Self {
        self.quit_key = Some(char_c as i32);
        self
    }

    pub fn build(self) -> Result<CameraHandler> {
        let idx = self
            .video_idx
            .ok_or_else(|| anyhow::anyhow!("Video index has to bet set"))?;
        let camera = VideoCapture::new(idx, CAP_ANY)
            .with_context(|| format!("Failed to open camera index {} check dev", idx))?;

        match &self.display_window {
            Some(name) => {
                let shit = name.clone();
                highgui::named_window(&shit, highgui::WINDOW_AUTOSIZE)?
            }
            None => {
                println!("No display window for camera")
            }
        }
        let quit_key = Arc::new(Mutex::new(self.quit_key.map_or('q' as i32, |f| f)));
        Ok(CameraHandler {
            inner: Arc::new(Mutex::new(CameraHandlerInner {
                camera,
                display_window: self.display_window,
                quit_key,
            })),
        })
    }
}
