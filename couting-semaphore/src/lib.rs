use std::sync::{Condvar, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct CountingSemaphoreWithCondvar {
    mutex: Mutex<usize>,
    condvar: Condvar
}

impl CountingSemaphoreWithCondvar{
    pub fn new(total_resources: usize) -> Self{
        CountingSemaphoreWithCondvar{
            mutex: Mutex::new(total_resources),
            condvar: Condvar::new()
        }
    }

    pub fn wait(&self){
        let mut available_resources = self.mutex.lock().unwrap();

        // it waits while available resources are zero, then it acquires an available resource
        available_resources = self.condvar.wait_while(available_resources, |available_resources| *available_resources == 0).unwrap();

        *available_resources-= 1;
    }

    pub fn signal(&self){
        let mut available_resources = self.mutex.lock().unwrap();
        // It signals that a resource is becoming available because it is releasing it
        *available_resources+= 1;
        self.condvar.notify_one();
    }
}


/**************************************************************************************************/


pub struct CountingSemaphoreWithChannel {
    available_resources: Mutex<usize>,
    sender: Mutex<Sender<()>>,
    receiver: Mutex<Receiver<()>>,
}

impl CountingSemaphoreWithChannel{
    pub fn new(total_resources: usize) -> Self{
        let (sender, receiver)= channel();
        CountingSemaphoreWithChannel{
            available_resources: Mutex::new(total_resources),
            sender: Mutex::new(sender),
            receiver: Mutex::new(receiver),
        }
    }

    pub fn wait(&self){
        let mut available_resources= self.available_resources.lock().unwrap();
        // If there are not available resources, it waits, else simply decrease the available resources
        // because it is acquiring one.
        if *available_resources== 0{
            // It drops the MutexGuard to release the lock, so to leave the possibility to acquire it
            // to other threads, during its waiting
            drop(available_resources);
            // Block it (the thread), waiting for a response
            self.receiver.lock().unwrap().recv().unwrap();
            // After be unblocked, it reacquires the lock to decrease the available resources because
            // now it is acquiring one.
            let mut available_resources= self.available_resources.lock().unwrap();
            *available_resources-= 1;
        }else{
            *available_resources-= 1;
        }
    }

    pub fn signal(&self){
        let mut available_resources= self.available_resources.lock().unwrap();
        // It signals that a resource is becoming available because it is releasing it
        *available_resources+= 1;
        self.sender.lock().unwrap().send(()).unwrap();
    }
}