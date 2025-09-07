use crossbeam::channel::Receiver;
use opencv::highgui;
use std::sync::Arc;

use crate::EventServer;
use crate::Message;

pub fn display_video(receiver: Receiver<Message>, event_server: Arc<EventServer>) {
    while let Ok(msg) = receiver.recv() {
        match msg {
            Message::Frame(frame) => {
                highgui::imshow("Camera1", &frame).unwrap();
                if highgui::wait_key(1).unwrap() == 'q' as i32 {
                    event_server.broadcast(Message::Stop);
                }
            }
            Message::Stop => break,
            _ => continue, // ignore the rest of messages
        }
    }
}
