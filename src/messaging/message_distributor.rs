use anyhow::Result;
use crossbeam::channel::Sender;
use crossbeam_skiplist::SkipMap;
use opencv::prelude::Mat;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ChannelID {
    Camera(u32),
    CameraStopper,
    Interpreter,
    WindowDisplay,
}
#[derive(Debug, Clone)]
pub enum Message {
    Frame(Mat),
    Start,
    Stop,
}

#[derive(Default)]
pub struct EventServer {
    registry: SkipMap<ChannelID, Sender<Message>>,
}

impl EventServer {
    pub fn register_msg(&self, id: ChannelID, msg: Sender<Message>) -> Result<()> {
        if let Some(_val) = self.registry.get(&id) {
            return Err(anyhow::anyhow!(
                "Message ID used twice overrinding current receiver"
            ));
        }
        self.registry.insert(id, msg);
        Ok(())
    }

    pub fn send(&self, id: &ChannelID, msg: Message) {
        if let Some(tx) = self.registry.get(id) {
            let _ = tx.value().send(msg);
        }
    }
    pub fn broadcast(&self, msg: Message) {
        for entry in self.registry.iter() {
            entry.value().send(msg.clone()).unwrap()
        }
    }
}

#[test]
fn test_message_sending() {
    use crossbeam::channel::bounded;
    use std::thread;

    let delivery_man = EventServer::default();
    let (tx, rx) = bounded(2);
    let (tx2, rx2) = bounded(2);

    let matthew = Message::Frame(Mat::default());

    delivery_man
        .register_msg(ChannelID::Interpreter, tx)
        .unwrap();
    delivery_man
        .register_msg(ChannelID::WindowDisplay, tx2)
        .unwrap();

    let t1 = thread::spawn(move || {
        let mut i = 0;
        while let Ok(msg) = rx.recv() {
            assert!(i < 10);
            if matches!(msg, Message::Stop) {
                break;
            }
            i += 1
        }
    });

    let t2 = thread::spawn(move || {
        assert!(rx2.recv().is_ok());
    });

    for _ in 0..8 {
        delivery_man.send(&ChannelID::Interpreter, matthew.clone());
    }
    delivery_man.broadcast(Message::Stop);

    t2.join().unwrap();
    t1.join().unwrap();
}
