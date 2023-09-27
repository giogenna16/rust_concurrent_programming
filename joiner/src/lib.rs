use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Condvar, Mutex};
use std::thread;

pub struct Joiner{
    number_of_producers: usize,
    producers_senders: Mutex<Vec<Sender<()>>>,
    producers_receivers:  Vec<Mutex<Receiver<()>>>,
    produced_values: Mutex<HashMap<usize, f64>>
}

impl Joiner{
    pub fn new(number_of_producers: usize)-> Self{
        let mut producers_senders= Vec::new();
        let mut producers_receivers= Vec::new();
        for _ in 0..number_of_producers{
            let (sender, receiver)= channel();
            producers_senders.push(sender);
            producers_receivers.push(Mutex::new(receiver));
        }

        Joiner{
            number_of_producers,
            producers_senders: Mutex::new(producers_senders),
            producers_receivers,
            produced_values: Mutex::new(HashMap::new())
        }
    }

    pub fn supply(&self, index: usize, produced_value: f64)-> HashMap<usize, f64>{
        let mut produced_values_locked= self.produced_values.lock().unwrap();
        // Insert the value in the Map
        produced_values_locked.entry(index)
            .and_modify(|value| {*value= produced_value})
            .or_insert(produced_value);
        // The index-th thread drops the MutexGuard to explicitly release the lock before blocking itself
        drop(produced_values_locked);

        // Send a message to the other producers, to notify the index-th producer produces the value
        for i in 0..self.number_of_producers{
            let producers_senders= self.producers_senders.lock().unwrap();
            if i != index{
                producers_senders[i].send(()).unwrap();
            }
            drop(producers_senders);
        }

        // Block the index-th producer until all the other producers produce their value
        for _ in 0..self.number_of_producers-1{
            self.producers_receivers[index].lock().unwrap().recv().unwrap()
        }

        // Reacquire the lock to the Map to return it
        return self.produced_values.lock().unwrap().clone();
    }
}


/**************************************************************************************************/


pub struct JoinerCvar{
    producers_number: usize,
    mutex: Mutex<(usize, usize, HashMap<usize, f64>)>,
    cvar: Condvar
}

impl JoinerCvar{
    pub fn new(producers_number: usize)-> Self{
        Self{
            producers_number,
            mutex: Mutex::new((0, 0, HashMap::new())),
            cvar: Condvar::new()
        }
    }

    pub fn supply(&self, index: usize, produced_value: f64)-> HashMap<usize, f64>{
        let mut mutex= self.mutex.lock().unwrap();
        mutex.0+= 1;
        let local_gen= mutex.1;
        mutex.2.entry(index)
            .and_modify(|p|*p= produced_value)
            .or_insert(produced_value);

        if mutex.0< self.producers_number{
            while local_gen== mutex.1{
                mutex= self.cvar.wait(mutex).unwrap();
            }
        }else{
            mutex.0=0;
            mutex.1+=1;
            self.cvar.notify_all();
        }
        return mutex.2.clone();
    }
}


/**************************************************************************************************/


pub struct JoinerWithConsumer{
    producers_receivers:  Vec<Mutex<Receiver<()>>>,
    consumer_sender:  Mutex<Sender<()>>,
    produced_values: Mutex<HashMap<usize, f64>>
}

impl JoinerWithConsumer {
    pub fn new(number_of_producers: usize) -> Self {
        let mut producers_senders = Vec::new();
        let mut producers_receivers = Vec::new();
        for _ in 0..number_of_producers {
            let (sender, receiver) = channel();
            producers_senders.push(sender);
            producers_receivers.push(Mutex::new(receiver));
        }

        let (consumer_sender, consumer_receiver) = channel();

        // consumer
        thread::spawn(move || {
            loop {
                // Waiting for the producers to produce the values
                for _ in 0..number_of_producers {
                    if consumer_receiver.recv().is_err(){
                        return;
                    }
                }

                // Send a response to the producers to unblock them
                for i in 0..number_of_producers {
                    producers_senders[i].send(()).unwrap();
                }
            }
        });

        JoinerWithConsumer {
            producers_receivers,
            consumer_sender: Mutex::new(consumer_sender),
            produced_values: Mutex::new(HashMap::new())
        }
    }


    pub fn supply(&self, index: usize, produced_value: f64)-> HashMap<usize, f64>{
        let mut produced_values_locked = self.produced_values.lock().unwrap();
        // Insert the value in the Map
        produced_values_locked.entry(index)
            .and_modify(|value| { *value = produced_value })
            .or_insert(produced_value);
        // Drop the MutexGuard to explicitly release the lock before blocking the index-th thread
        drop(produced_values_locked);

        // Send a message to the consumer, to notify the index-th producer produces the value
        self.consumer_sender.lock().unwrap().send(()).unwrap();

        // Block the index-th producer until the consumer unblocks
        self.producers_receivers[index].lock().unwrap().recv().unwrap();

        return self.produced_values.lock().unwrap().clone();
    }
}

