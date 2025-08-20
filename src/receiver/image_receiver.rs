use std::{sync::{mpsc::{Receiver,Sender},Arc,Mutex}, thread::{self, JoinHandle}};
use opencv::prelude::*;
use anyhow::Result;
use std::sync::mpsc;

use crate::{wait_for_start_or_stop, MatConverter, Start, Stop, ThreadOperation};

pub struct ImageReceiver{
    rx: Receiver<Mat>,
    tx: Sender<Mat>, // no used
    sync : Arc<Mutex<ThreadOperation>>
}

/// Getters
impl ImageReceiver {
    pub fn get_transmitter(&self) -> Sender<Mat>{
        self.tx.clone()
    }
}

/// Main functions
impl ImageReceiver {
    fn run_loop(&mut self)-> Result<()>{
        let mut mat_converter = MatConverter::default();
        loop {
            match *self.sync.lock().unwrap() {
                ThreadOperation::STOP => break,
                _ => (),
            };
            let mut frame = self.rx.recv()?;
            mat_converter.mat_to_tensor(&mut frame)?;
        }
        Ok(())
    }
    fn process_frame(&mut self) -> Result<()>{
            wait_for_start_or_stop(self.sync.clone());
            self.run_loop()?;
        Ok(())
    }
}

impl Start for ImageReceiver{
    fn start(&mut self) {
        *self.sync.lock().unwrap() = ThreadOperation::START;
        self.process_frame().unwrap();
    }
}

impl Stop for ImageReceiver{
    fn stop(&mut self) {
        *self.sync.lock().unwrap() = ThreadOperation::STOP;
    }
}

/// Constructor
impl ImageReceiver {
    pub fn new() -> Self{
        let (tx, rx) = mpsc::channel();
        Self{ rx, tx, sync: Arc::new(Mutex::new(ThreadOperation::default()))}
    }
}
