use std::{sync::{mpsc::{Receiver,Sender},Arc,Mutex}};
use opencv::prelude::*;
use anyhow::Result;
use std::thread::JoinHandle;
use std::thread;
use std::sync::mpsc;


use crate::{wait_for_start_or_stop, MatConverter, Start, Stop, ThreadOperation};

pub struct ImageReceiver{
    // rx: Receiver<Mat>,
    tx: Option<Sender<Mat>>, // no used
    sync : Arc<Mutex<ThreadOperation>>,
    op_tr : Vec<JoinHandle<()>>
}

/// Getters
impl ImageReceiver {
    pub fn get_transmitter(&self) -> Sender<Mat>{
        match &self.tx {
            Some( tx) => tx.clone(),
            None => panic!("Transmitter called before  starting receiver thread")
        }
    }
}

/// Main functions
impl ImageReceiver {
    fn run_loop(rx : Receiver<Mat>, sync : Arc<Mutex<ThreadOperation>>)-> Result<()>{
        let mut mat_converter = MatConverter::default();
        loop {
            match *sync.lock().unwrap() {
                ThreadOperation::STOP => break,
                _ => (),
            };
            let mut frame = rx.recv()?;
            mat_converter.mat_to_tensor(&mut frame)?;
            eprintln!("We are sending data");
        }
        Ok(())
    }
    fn process_frame(&mut self) -> Result<()>{
        let ( tx , rx ) = mpsc::channel(); 
        self.tx = Some(tx);
        let sync_ptr = self.sync.clone();
        self.op_tr.push(
            thread::spawn( move||{
            wait_for_start_or_stop(sync_ptr.clone());
            ImageReceiver::run_loop(rx,sync_ptr.clone()).unwrap();
        }));
        Ok(())
    }
}

impl Start for ImageReceiver{
    fn start(&mut self) {
        *self.sync.lock().unwrap() = ThreadOperation::START;
    }
}

impl Stop for ImageReceiver{
    fn stop(&mut self) {
        *self.sync.lock().unwrap() = ThreadOperation::STOP;
        for th in self.op_tr.drain(..) {
            th.join().unwrap();
        }
    }
}

/// Constructor
impl ImageReceiver {
    pub fn new() -> Self{
        Self{ tx: None, sync: Arc::new(Mutex::new(ThreadOperation::default())), op_tr : Vec::new()}
    }
    pub fn initialze(&mut self) -> &Self {
        self.process_frame().unwrap();
        self
    }
}
