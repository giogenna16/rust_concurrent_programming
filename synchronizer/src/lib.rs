use std::sync::{Condvar, Mutex};

pub struct Synchronizer{
    process:  fn(f64, f64)->f64,
    mutex: Mutex<(usize, usize, f64)>,
    condvar: Condvar

}

impl Synchronizer{
    pub fn new(process: fn(f64, f64)->f64)->Self{
        Synchronizer{
            process,
            mutex: Mutex::new((0 ,0, 0.0)), //count, generation, data
            condvar: Condvar::new()
        }
    }

    pub fn data_from_first_port(&self, f1: f64){
        let mut mutex= self.mutex.lock().unwrap();
        let local_generation= mutex.1;
        mutex.0+=1;

        if mutex.0 < 2{
            mutex.2= f1;
            while local_generation== mutex.1{
                mutex= self.condvar.wait(mutex).unwrap();
            }
        }else{
            let f= self.process;
            let result= f(f1, mutex.2);
            println!("Result: {}", result);
            mutex.0= 0;
            mutex.1+= 1;
            mutex.2= 0.0;
            self.condvar.notify_one()
        }
    }

    pub fn data_from_second_port(&self, f2: f64){
        let mut mutex= self.mutex.lock().unwrap();
        let local_generation= mutex.1;
        mutex.0+=1;

        if mutex.0 < 2{
            mutex.2= f2;
            while local_generation== mutex.1{
                mutex= self.condvar.wait(mutex).unwrap();
            }
        }else{
            let f= self.process;
            let result= f(f2, mutex.2);
            println!("Result: {}", result);
            mutex.0= 0;
            mutex.1+= 1;
            mutex.2= 0.0;
            self.condvar.notify_one()
        }
    }
}