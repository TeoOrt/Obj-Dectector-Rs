use std::sync::{Arc,Mutex};
use std::time::Duration;
use std::thread;

#[derive(Default)]
pub enum ThreadOperation {
    START,
    #[default]
    IDLE,
    STOP,
}


pub trait Start {
    fn start(&mut self);
}

pub trait Stop{
    fn stop(&mut self);
}
    pub fn wait_for_start_or_stop(operation: Arc<Mutex<ThreadOperation>>) {
        loop {
            match *operation.lock().unwrap() {
                ThreadOperation::START => break,
                ThreadOperation::STOP => return,
                ThreadOperation::IDLE => thread::sleep(Duration::new(0, 100)),
            }
        }
    }
