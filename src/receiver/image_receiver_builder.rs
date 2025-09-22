use crate::{EventServer, Message, labels::Labels};
use crossbeam::channel::Receiver;
use std::sync::Arc;

pub struct ImageRecvBuilder {
    sync_recv: Receiver<Message>,
    event_server: Arc<EventServer>,
    frame_receivers: Vec<Receiver<Message>>,
    labels: Arc<Labels>,
}

impl ImageRecvBuilder {
    pub fn with_labels(mut self, label: Arc<Labels>) -> Self {
        self.labels = label;
        self
    }
    pub fn with_event_server(mut self, event_server: Arc<EventServer>) -> Self {
        self.event_server = event_server;
        self
    }
    pub fn with_frame_receivers(mut self, frame_receivers: Vec<Receiver<Message>>) -> Self {
        self.frame_receivers = frame_receivers;
        self
    }
}

// pub struct ImageReceiver {
//     sync_recv: Receiver<Message>,
//     #[allow(unused)]
//     event_server: Arc<EventServer>,
//     frame_receivers: Vec<Receiver<Message>>,
//     threads: Vec<JoinHandle<()>>,
// }
