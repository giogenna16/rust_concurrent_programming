use std::sync::Arc;
use std::thread;
use executor::Executor;

fn main() {
    let executor= Arc::new(Executor::new());
    thread::scope(|s|{
        for i in 0..50{
            let executor_clone= executor.clone();
            s.spawn(move ||{
                if i== 4{
                    let close= executor_clone.shutdown();
                    if close.is_some(){
                        println!("Thread {} shut down the executor", i);
                    }else{
                        println!("The executor is already closed");
                    }
                }else{
                    println!("Thread {} submits a function", i);
                    let x= 10;
                    let y= 20;
                    let result= executor_clone.submit(Box::new(move || {
                        println!("Thread {}: executing the function", i);
                        let _ = x+y;
                        return i ;
                    }));
                    if result.is_some(){
                        println!("Thread {}: the result is: {}", i, result.unwrap());
                    }else{
                        println!("Thread {}: the executor is closed!", i);
                    }
                }
            });
        }


    });
}
