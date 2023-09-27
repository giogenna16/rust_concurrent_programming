use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Dispatcher<Msg: Send+ Clone>{
    senders: Mutex<Vec<Sender<Msg>>>
}

impl<Msg: Send+ Clone> Dispatcher<Msg> {
    pub fn new()-> Self{
        Dispatcher{
            senders: Mutex::new(Vec::new())
        }
    }

    pub fn dispatch(&self, msg: Msg) {
        let mut senders = self.senders.lock().unwrap();
        for i in (0..senders.len()).rev(){
            if senders[i].send(msg.clone()).is_err(){
                senders.remove(i);
            }
        }
    }

    pub fn subscribe(&self)-> Subscription<Msg> {
        let (sender, receiver)= channel();
        self.senders.lock().unwrap().push(sender);
        return Subscription::new(receiver)
    }
}

/**************************************************************************************************/

pub struct Subscription<Msg: Send+ Clone>{
    receiver: Receiver<Msg>
}

impl<Msg: Send+ Clone> Subscription<Msg>{
    pub fn new(receiver: Receiver<Msg>)-> Self{
        Subscription{
            receiver
        }
    }

    pub fn read(&self) -> Option<Msg> {
        let message = self.receiver.recv();
        return match message {
            Ok(message) => Some(message),
            Err(_) => None,
        };
    }
}