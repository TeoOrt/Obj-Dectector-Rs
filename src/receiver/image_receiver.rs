use crate::{
    open_onnx_model, wait_for_start_or_stop, MatConverter, MessageDist, RtSync, TensorPredictor, ThreadOperation
};
use anyhow::Result;
use opencv::prelude::*;
use std::sync::mpsc;
use std::sync::{
    Arc, Mutex,
    mpsc::{Receiver, Sender},
};
use std::thread;
use std::thread::JoinHandle;

pub struct ImageReceiver {
    tx: Option<Sender<Mat>>, // no used
    sync: Arc<Mutex<ThreadOperation>>,
    event_sender : Arc<MessageDist>,
    op_tr: Vec<JoinHandle<()>>,
}

/// Getters
impl ImageReceiver {
    pub fn get_transmitter(&self) -> Sender<Mat> {
        match &self.tx {
            Some(tx) => tx.clone(),
            None => panic!("Transmitter called before  starting receiver thread"),
        }
    }
}

/// Main functions
impl ImageReceiver {
    fn run_loop(rx: Receiver<Mat>, sync: Arc<Mutex<ThreadOperation>>) -> Result<()> {
        let mut mat_converter = MatConverter::default();
        let mut buffer = Vec::new();
        let model = open_onnx_model("yolov5s.onnx")?;
        let mut predictor = TensorPredictor::default();
        buffer.reserve(20);
        loop {
            match *sync.lock().unwrap() {
                ThreadOperation::STOP => break,
                _ => (),
            };
            let frame = rx.recv()?;
            buffer.push(frame);

            if buffer.len() == 16{
                let tensor = mat_converter.mats_to_tensor::<u8>(&buffer)?;
                predictor.interpret_message(&model, tensor)?;
                eprintln!("We are sending data");
                buffer.clear();
            }
        }
        Ok(())
    }
    fn process_frame(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        self.tx = Some(tx);
        let sync_ptr = self.sync.clone();
        self.op_tr.push(thread::spawn(move || {
            wait_for_start_or_stop(sync_ptr.clone());
            ImageReceiver::run_loop(rx, sync_ptr.clone()).unwrap();
        }));
        Ok(())
    }
}

impl RtSync for ImageReceiver {
    fn start(&mut self) {
        *self.sync.lock().unwrap() = ThreadOperation::START;
    }
    fn stop(&mut self) {
        *self.sync.lock().unwrap() = ThreadOperation::STOP;
        for th in self.op_tr.drain(..) {
            th.join().unwrap();
        }
    }
}

/// Constructor
impl ImageReceiver {
    pub fn new() -> Self {
        Self {
            tx: None,
            sync: Arc::new(Mutex::new(ThreadOperation::default())),
            op_tr: Vec::new(),
        }
    }
    pub fn initialze(&mut self) -> &Self {
        self.process_frame().unwrap();
        self
    }
}
