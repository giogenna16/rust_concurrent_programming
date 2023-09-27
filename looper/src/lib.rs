use std::fmt::{Display};
use std::sync::Mutex;
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub struct Message<T: Clone + Display + Send + 'static>{
    message: T
}

impl<T: Clone + Display + Send + 'static> Message<T>{
    pub fn new(message: T)-> Self{
        Message{
            message
        }
    }

    pub fn content(&self)-> T{
        return self.message.clone();
    }
}

/**************************************************************************************************/

pub struct Looper<T: Clone + Display + Send + 'static>{
    sender: Mutex<Sender<Message<T>>>
}

impl<T: Clone + Display + Send + 'static> Looper<T>{
    pub fn new(process: fn(Message<T>), cleanup: fn())-> Self{
        let (sender, receiver)= channel::<Message<T>>();
        let (sender,  receiver)= (Mutex::new(sender), Mutex::new(receiver));

        thread::spawn(move||{
            loop{
                let message = receiver.lock().unwrap().recv();
                if message.is_ok(){
                    let message= message.unwrap();
                    println!("Received message: {}", message.content());
                    process(message);
                }else{
                    cleanup();
                    return; //exit from the closure, so delete also the thread
                }
            }
        });

        Looper{
            sender
        }
    }

    pub fn send(&self, message: Message<T>){
        self.sender.lock().unwrap().send(message).unwrap();
    }
}


/**************************************************************************************************/

pub fn process<T: Display+Clone+Send+'static>(message: Message<T>){
    println!("The looper is elaborating the message: '{}'", message.content());
}

pub fn cleanup(){
    println!("The Looper is about to be destroyed");
}
