use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub struct Processor<T: Send + 'static>{
    cvar: Arc<Condvar>,
    mutex: Arc<Mutex<(VecDeque<T>, bool, bool)>>, // (FIFO queue, closed, wait_for_consumer)
}

impl<T: Send + 'static> Processor<T>{
    pub fn new()-> Self{
        let cvar= Arc::new(Condvar::new());
        let mutex= Arc::new(Mutex::new((VecDeque::new(), false, true)));

        let mutex_clone= mutex.clone();
        let cvar_clone= cvar.clone();
        thread::spawn(move ||{
            loop{
                let mut mutex= mutex_clone.lock().unwrap();
                let local_len= mutex.0.len();

                // If the length of the queue does not change (so there are not new elements) and
                // the channel is still open (not closed), wait for a new element
                while local_len== mutex.0.len() && !mutex.1{
                    mutex= cvar_clone.wait(mutex).unwrap();
                }

                // If there are still elements in the queue, process the first; else, if the channel
                // is closed, send an advice to the thread on the close function
                if mutex.0.len() > 0{
                    let t= mutex.0.pop_front();
                    drop(mutex);
                    simulate_execution(t);
                }else{
                    // If it is closed (maybe redundant)
                    if mutex.1 {
                        mutex.2= false;
                        drop(mutex);
                        cvar_clone.notify_one();
                    }
                }
            }
        });

        Processor{
            mutex,
            cvar
        }
    }

    pub fn send(&self, t: T) -> Option<()>{
        let mut mutex= self.mutex.lock().unwrap();
        return if mutex.1 {
            println!("Processor already closed!");
            None
        } else {
            // Push the new element in the queue and notify the consumer
            mutex.0.push_back(t);
            drop(mutex);
            self.cvar.notify_one();
            Some(())
        }
    }

    pub fn close(&self){
        let mut mutex= self.mutex.lock().unwrap();
        // If it is not already closed
        if !mutex.1{
            // Set closed to true and notify the change to the consumer
            mutex.1= true;
            drop(mutex);
            self.cvar.notify_one();
            // The close() method will not return until the consumer has consumed all the objects
            // already received
            let mut mutex= self.mutex.lock().unwrap();
            while mutex.2{
                mutex= self.cvar.wait(mutex).unwrap();
            }
            println!("The consumer finished!"); // for debugging
        }
    }
}

fn simulate_execution<T>(_t: T){
    // Random code
    println!("Simulating an execution...");
    let mut j= 0;
    for i  in 0..1_000{
        j+= i;
        j-= i;
        j*= j;
    }
    println!("...Simulation finished!");
}