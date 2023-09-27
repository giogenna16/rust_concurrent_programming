use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub struct SingleThreadExecutor{
    mutex: Arc<Mutex<(VecDeque<Box<dyn FnOnce() + Send>>, bool, bool, bool)>>, //FIFO queue, closed, joined, ended
    cvar: Arc<Condvar>
}

impl SingleThreadExecutor{
    pub fn new() -> Self{
        let mutex= Arc::new(Mutex::new((VecDeque::<Box<dyn FnOnce() + Send>>::new(), false, false, false)));
        let cvar= Arc::new(Condvar::new());

        let mutex_clone= mutex.clone();
        let cvar_clone= cvar.clone();
        thread::spawn(move||{
            loop {
                let mut mutex = mutex_clone.lock().unwrap();
                let local_length = mutex.0.len();

                // If the length of the queue does not change (so there are not new elements) and
                // the channel is still not joined, wait for a new element
                while local_length == mutex.0.len() && !mutex.2{
                    mutex = cvar_clone.wait(mutex).unwrap();
                }

                // If there are still elements in the queue, process the first; else, if the channel
                // is joined, send an advice to notify the end to the thread on the join function
                if mutex.0.len() > 0 {
                    let option_f = mutex.0.pop_front();
                    drop(mutex);
                    if option_f.is_some() {
                        let f = option_f.unwrap();
                        f(); //execute the selected task
                    }
                }else{
                    // If the channel is joined
                    if mutex.2{
                        mutex.3 = true;
                        drop(mutex);
                        cvar_clone.notify_one();
                    }
                }
            }
        });

        SingleThreadExecutor{
            mutex,
            cvar
        }
    }

    pub fn submit(&self, f: Box<dyn FnOnce() + Send>)-> Option<()>{
        let mut mutex=  self.mutex.lock().unwrap();
        // If it is open, push the new function and notify the executor
        return if mutex.1 {
            println!("The executor is closed");
            None
        } else {
            mutex.0.push_back(f);
            drop(mutex);
            self.cvar.notify_one();
            Some(())
        }
    }

    pub fn close(&self){
        // Close the channel for the senders
        let mut mutex= self.mutex.lock().unwrap();
        mutex.1= true;
    }

    pub fn join(&self){
        // Set the joined flag to true and notify the executor
        let mut mutex= self.mutex.lock().unwrap();
        if !mutex.2{
            mutex.2= true;
            drop(mutex);
            self.cvar.notify_one();

            // Wait for the end of the executor
            let mut mutex= self.mutex.lock().unwrap();
            while !mutex.3{
                mutex= self.cvar.wait(mutex).unwrap();
            }
        }
    }

}