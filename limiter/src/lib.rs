use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

/**
 Limiter is a counting semaphore but, instead of having two function, wait() and signal(), to
 delimit the critical section, here there is only one function, wait_and_signal(), and the critical
 operations are executed directly inside this function. This is implemented using condvar
*/
pub struct Limiter{
    mutex: Mutex<usize>,
    condvar: Condvar,
}

impl Limiter{
    pub fn new(threshold: usize)->Self{
        Limiter{
            mutex: Mutex::new(threshold),
            condvar: Condvar::new()
        }
    }

    pub fn wait_and_signal(&self, i: usize){
       let mut available_resources= self.mutex.lock().unwrap();
        println!("Thread {} is waiting for a resource", i);
        while *available_resources == 0{
            available_resources = self.condvar.wait(available_resources).unwrap();
        }
        *available_resources -= 1;
        drop(available_resources); // release the lock

        println!("Thread {} acquired the resource", i);
        thread::sleep(Duration::from_secs(5)); // simulate an execution

        let mut available_resources = self.mutex.lock().unwrap(); // reacquire the lock
        *available_resources += 1;
        println!("Thread {} released the resources", i);
        self.condvar.notify_one();
    }
}


/**************************************************************************************************/

/**
 Limiter is a counting semaphore but, instead of having two function, wait() and signal(), to
 delimit the critical section, here there is only one function, wait_and_signal(), and the critical
 operations are executed directly inside this function. This is implemented using channel.
*/
pub struct LimiterChannel{
    available_resources: Mutex<usize>,
    sender: Mutex<Sender<()>>,
    receiver: Mutex<Receiver<()>>
}

impl LimiterChannel{
    pub fn new(threshold: usize)->Self{
        let(sender, receiver)= channel();
        LimiterChannel{
            available_resources: Mutex::new(threshold),
            sender: Mutex::new(sender),
            receiver: Mutex::new(receiver),
        }
    }

    pub fn wait_and_signal(&self, i: usize){
        let mut available_resources= self.available_resources.lock().unwrap();
        println!("Thread {} is waiting for a resource", i);
        if *available_resources == 0{
            drop(available_resources); // release the lock
            self.receiver.lock().unwrap().recv().unwrap();
            let mut available_resources= self.available_resources.lock().unwrap();// reacquire the lock
            *available_resources -= 1;
        }else{
            *available_resources -= 1;
            drop(available_resources); // release the lock
        }

        println!("Thread {} acquired the resource", i);
        thread::sleep(Duration::from_secs(5)); // simulate an execution

        let mut available_resources= self.available_resources.lock().unwrap();// reacquire the lock
        *available_resources+= 1;
        println!("Thread {} released the resources", i);
        self.sender.lock().unwrap().send(()).unwrap();
    }
}