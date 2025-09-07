use crate::{ChannelID, EventServer, Message};

use super::Camera;
use anyhow::Result;
use crossbeam::channel::Receiver;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

pub struct CameraOperator {
    cameras: Vec<Camera>,
    thread_handles: Vec<JoinHandle<()>>,
    event_server: Arc<EventServer>,
    sync_recv: Receiver<Message>,
}

/// Helper functions
impl CameraOperator {
    fn start_loop(
        camera: Camera,
        event_server: Arc<EventServer>,
        sync_recv: Receiver<Message>,
        camera_id: usize,
    ) -> Result<()> {
        while !matches!(sync_recv.recv()?, Message::Start) {} // waiting for start message
        loop {
            let mut cam = camera.inner.lock().unwrap();
            let frame = cam.get_frame()?;
            event_server.send(&ChannelID::Camera(camera_id as u32), Message::Frame(frame));
            match sync_recv.try_recv() {
                Ok(received) => {
                    if matches!(received, Message::Stop) {
                        break;
                    }
                }
                Err(_) => continue,
            };
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
        self.thread_handles = self
            .cameras
            .par_iter()
            .enumerate()
            .map(|(camera_id, camera)| {
                // this is stupid
                let cam_clone = camera.clone();
                let es_ptr = self.event_server.clone();
                let sync_ptr = self.sync_recv.clone();
                thread::spawn(move || {
                    let cam = cam_clone.clone();
                    CameraOperator::start_loop(
                        cam.clone(),
                        es_ptr.clone(),
                        sync_ptr.clone(),
                        camera_id,
                    )
                    .unwrap()
                })
            })
            .collect();
        Ok(())
    }
}

/// Constructors
impl CameraOperator {
    pub fn new(camera_list: Vec<Camera>, event_server: Arc<EventServer>) -> Self {
        // sync
        let (sync_sender, sync_recv) = crossbeam::channel::unbounded();
        event_server
            .register_msg(ChannelID::CameraStopper, sync_sender)
            .unwrap();

        Self {
            cameras: camera_list,
            thread_handles: Vec::new(),
            event_server,
            sync_recv,
        }
    }
    pub fn initialze(&mut self) -> &Self {
        self.process_camera_frames().unwrap();
        self
    }
}

impl Drop for CameraOperator {
    fn drop(&mut self) {
        eprintln!("Joining Threads for CameraOperator");
        for threads in self.thread_handles.drain(0..) {
            threads.join().unwrap();
            eprintln!("Joined thread");
        }
    }
}
