


use crossbeam_skiplist::SkipMap;
use opencv::prelude::Mat;
use crossbeam::channel::{Sender };
use anyhow::Result;

#[derive(Debug, Clone, PartialEq , Eq, Ord, PartialOrd, Hash)]
pub enum MessageID {
    Camera(u32),
    Interpreter,
    WindowDisplay,
}
#[derive(Debug,Clone)]
pub enum Message {
    Frame(Mat),
    Start,
    Stop 
}


#[derive(Default)]
pub struct MessageDist{
    registry : SkipMap<MessageID,Sender<Message>>
}


impl MessageDist{
    pub fn register_msg(&self, id : MessageID , msg: Sender<Message>) -> Result<()>{
        if let Some(_val) = self.registry.get(&id) {
            return  Err(anyhow::anyhow!("Message ID used twice overrinding current receiver"));
        }
        self.registry.insert(id, msg);
        Ok(())
    }

    pub fn send(&self , id: &MessageID, msg: Message){
        if let Some(tx) = self.registry.get(id){
            let _ = tx.value().send(msg);
        }
    }
    pub fn broadcast(&self, msg: Message){
        for entry in self.registry.iter(){
            entry.value().send(msg.clone()).unwrap()
        }
    }
}




#[test]
fn test_message_sending(){
    use std::thread;
    use crossbeam::channel::bounded;

    let delivery_man = MessageDist::default();
    let (tx,rx) = bounded(2);
    let (tx2,rx2) = bounded(2);

    let matthew = Message::Frame(Mat::default());


    delivery_man.register_msg(MessageID::Interpreter, tx).unwrap();
    delivery_man.register_msg(MessageID::WindowDisplay, tx2).unwrap();

    let t1 = thread::spawn(move || {
        let mut i = 0;
        while let Ok(msg) = rx.recv() {
            assert!( i < 10);
            if matches!(msg,Message::Stop){
                break;
            }
            i+=1
        }
    });

    let t2 = thread::spawn(move || {
        assert!( rx2.recv().is_ok() );
    });

    for _ in 0..8{
        delivery_man.send(&MessageID::Interpreter, matthew.clone());
    }
    delivery_man.broadcast(Message::Stop);

    t2.join().unwrap();
    t1.join().unwrap();

}



