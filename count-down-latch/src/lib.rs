use std::sync::{Condvar, Mutex};

pub struct CountDownLatch{
    count: Mutex<usize>,
    cvar: Condvar
}

impl CountDownLatch{

    pub fn new(n: usize)-> Self{
        CountDownLatch{
            count: Mutex::new(n),
            cvar: Condvar::new(),

        }
    }

    pub fn a_wait(&self){
        let mut count= self.count.lock().unwrap();

        while *count > 0 {
            count= self.cvar.wait(count).unwrap();
        }
    }

    pub fn count_down(&self){
        let mut count= self.count.lock().unwrap();

        if *count > 0{
            *count -= 1;

            if *count == 0{
                self.cvar.notify_all();
            }
        }
    }
}