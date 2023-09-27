use std::sync::{Condvar, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Exchanger<T: Default + Clone>{
    mutex: Mutex<(usize, T)>,
    condvar: Condvar
}

impl<T: Default + Clone> Exchanger<T> {

    pub fn new()-> Self{
        Exchanger{
            mutex: Mutex::new((usize::default(), T::default())),
            condvar: Condvar::new()
        }
    }

    pub fn exchange(&self, id: usize, message: T)-> T{
        let mut mutex= self.mutex.lock().unwrap();
        while mutex.0 == id{
            mutex= self.condvar.wait(mutex).unwrap();
        }

        mutex.0= id;
        let received_message= mutex.1.clone();
        mutex.1= message;

        self.condvar.notify_one();

        return received_message;
    }
}


/**************************************************************************************************/


pub struct ExchangerChannel<T>{
    senders:  [Mutex<Sender<T>>; 2],
    receivers: [Mutex<Receiver<T>>; 2],
}

impl<T> ExchangerChannel<T> {

    pub fn new()-> Self{
        let (sender0, receiver0)= channel::<T>();
        let (sender1, receiver1)= channel::<T>();
        let senders= [Mutex::new(sender0), Mutex::new(sender1)];
        let receivers= [Mutex::new(receiver0), Mutex::new(receiver1)];
        ExchangerChannel{
            senders,
            receivers,
        }
    }

    pub fn exchange(&self, i: usize, message: T)-> T{
        let j= (-(i as isize)+1) as usize; // i and j can be 0 or 1
        // Send the message to the other thread
        self.senders[j].lock().unwrap().send(message).unwrap();

        // Wait for the message sent by the other thread; when it arrives, return it
        return self.receivers[i].lock().unwrap().recv().unwrap();
    }
}


