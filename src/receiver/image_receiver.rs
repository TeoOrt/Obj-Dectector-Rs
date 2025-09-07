use crate::{ChannelID, EventServer, MatConverter, Message, TensorPredictor, open_onnx_model};
use anyhow::Result;
use crossbeam::channel::{Receiver, unbounded};
use opencv::core::Mat;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
// use opencv::prelude::*;
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

pub struct ImageReceiver {
    sync_recv: Receiver<Message>,
    #[allow(unused)]
    event_server: Arc<EventServer>,
    frame_receivers: Vec<Receiver<Message>>,
    threads: Vec<JoinHandle<()>>,
}

impl ImageReceiver {
    fn process_ml_algo(
        sync_recv: Receiver<Message>,
        frame_receivers: Vec<Receiver<Message>>,
    ) -> Result<()> {
        let mut mat_converter = MatConverter::default();
        let simplepan = open_onnx_model("yolov5s.onnx")?;
        let mut ml_processor = TensorPredictor::default();
        while !matches!(sync_recv.recv()?, Message::Start) {} // waiting for start message

        loop {
            let messages: Vec<Mat> = frame_receivers
                .iter()
                .filter_map(|recv| {
                    let message = recv.recv().unwrap();
                    match message {
                        Message::Frame(mat) => Some(mat),
                        _ => None,
                    }
                })
                .collect();
            println!("Size is {}", messages.len());
            let tensors = mat_converter.mats_to_tensor::<u8>(&messages)?;
            let _ = ml_processor.interpret_message(&simplepan, tensors)?;
            match sync_recv.try_recv() {
                Ok(received) => {
                    if matches!(received, Message::Stop) {
                        break;
                    }
                }
                _ => continue,
            }
        }

        Ok(())
    }
}

impl ImageReceiver {
    fn process_camera_frames(&mut self) -> Result<()> {
        let receiver_list = std::mem::take(&mut self.frame_receivers);
        let event_ptr = self.sync_recv.clone();
        self.threads.push(thread::spawn(move || {
            ImageReceiver::process_ml_algo(event_ptr.clone(), receiver_list).unwrap();
        }));
        Ok(())
    }
}

/// Constructor
impl ImageReceiver {
    pub fn new(event_server: Arc<EventServer>, frame_receivers: Vec<Receiver<Message>>) -> Self {
        let (tx, sync_recv) = unbounded();
        event_server
            .register_msg(ChannelID::Interpreter, tx)
            .unwrap();
        Self {
            sync_recv,
            event_server,
            frame_receivers,
            threads: Vec::new(),
        }
    }

    pub fn initialze(&mut self) {
        self.process_camera_frames().unwrap();
    }
}
