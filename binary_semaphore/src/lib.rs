use std::sync::{Condvar, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct BinarySemaphore{
    mutex: Mutex<bool>,
    cvar: Condvar
}

impl BinarySemaphore{
    pub fn new()->Self{
        BinarySemaphore{
            mutex: Mutex::new(false),
            cvar:  Condvar::new()
        }
    }

    pub fn wait(&self){
        let mut stop= self.mutex.lock().unwrap();
        while *stop{
            stop= self.cvar.wait(stop).unwrap();
        }
        *stop= true;
    }

    pub fn signal(&self){
        let mut stop= self.mutex.lock().unwrap();
        *stop= false;
        self.cvar.notify_one();
    }
}


pub struct BinarySemaphoreChannel{
    flag: Mutex<bool>,
    sender: Mutex<Sender<()>>,
    receiver: Mutex<Receiver<()>>
}

impl BinarySemaphoreChannel{
    pub fn new()->Self{
        let (sender, receiver)= channel();
        BinarySemaphoreChannel{
            flag: Mutex::new(false),
            sender: Mutex::new(sender),
            receiver: Mutex::new(receiver)
        }
    }

    pub fn wait(&self){
        let mut flag= self.flag.lock().unwrap();
        if *flag{
            drop(flag);
            self.receiver.lock().unwrap().recv().unwrap();
            let mut flag= self.flag.lock().unwrap();
            *flag= true;
        }else{
            *flag= true;
        }
    }

    pub fn signal(&self){
        let mut flag= self.flag.lock().unwrap();
        *flag= false;
        self.sender.lock().unwrap().send(()).unwrap();
    }
}