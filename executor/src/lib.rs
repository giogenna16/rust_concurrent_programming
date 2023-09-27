use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub struct Executor<T: Send+'static>{
    exec_sender: Mutex<Sender<Box<dyn FnOnce() -> T + Send>>>,
    result_receiver: Mutex<Receiver<T>>,
    closed: Arc<Mutex<bool>>
}

impl<T: Send+'static> Executor<T>{
    pub fn new()-> Self{
        let (exec_sender, exec_recv)= channel::<Box<dyn FnOnce() -> T + Send>>();
        let (result_sender, result_receiver)= channel::<T>();
        let closed= Arc::new(Mutex::new(false));

        let closed_clone= closed.clone();
        thread::spawn(move ||{
            loop{
                let closed= closed_clone.lock().unwrap();

                if !*closed{
                    drop(closed);
                    let f= exec_recv.recv().unwrap();
                    let t= f();
                    result_sender.send(t).unwrap();
                }else{
                    drop(closed);
                    while let Ok(f) = exec_recv.try_recv() {
                        let t= f();
                        result_sender.send(t).unwrap();
                    }
                }
            }
        });

        Executor{
            exec_sender: Mutex::new(exec_sender),
            result_receiver: Mutex::new(result_receiver),
            closed
        }
    }

    pub fn submit(&self, f: Box<dyn FnOnce() -> T + Send>) -> Option<T> {
        let closed= self.closed.lock().unwrap();
        return if !*closed{
            drop(closed);
            self.exec_sender.lock().unwrap().send(f).unwrap();
            let t= self.result_receiver.lock().unwrap().recv().unwrap();
            Some(t)
        } else {
            None
        }
    }

    pub fn shutdown(&self)-> Option<()>{
        let mut closed= self.closed.lock().unwrap();
        return if !*closed{
            *closed= true;
            Some(())
        }else{
            None
        }
    }
}
