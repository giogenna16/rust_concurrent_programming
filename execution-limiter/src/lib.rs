use std::sync::{Condvar, Mutex};

pub struct ExecutionLimiter{
    available_resources: Mutex<usize>,
    condvar: Condvar
}

impl ExecutionLimiter{
    pub fn new(total_resources: usize)-> Self{
        ExecutionLimiter{
            available_resources: Mutex::new(total_resources),
            condvar: Condvar::new()
        }
    }

    pub fn execute<R>(&self, f: fn()-> Result<R, ()>)-> Option<R>{
        let mut available_resources= self.available_resources.lock().unwrap();
        while *available_resources==0{
            available_resources= self.condvar.wait(available_resources).unwrap();
        }
        *available_resources-=1;
        drop(available_resources);

        let result= f();

        let mut available_resources= self.available_resources.lock().unwrap();
        *available_resources+=1;
        self.condvar.notify_one();

        return if result.is_ok(){
            result.ok()
        }else{
            None
        }
    }
}


/**************************************************************************************************/


// example function
pub fn heavy_computation() -> Result<u64, ()> {
    let mut result = 0;
    for _ in 0..200{
        for i in 0..1_000_000 {
            result += i;
        }
        for i in 0..1_000_000 {
            result -= i;
        }
    }
    return Ok(result);
}