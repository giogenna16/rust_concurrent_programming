use std::sync::Arc;
use std::thread;
use execution_limiter::{ExecutionLimiter, heavy_computation};

fn main() {
    let execution_limiter= Arc::new(ExecutionLimiter::new(3));

    thread::scope(|s|{
        for i in 0..8{
            let limiter_clone= Arc::clone(&execution_limiter);
            s.spawn(move ||{
                let result= limiter_clone.execute(||heavy_computation());
                println!("Thread {} computed a result equal to {}", i, result.unwrap());
            });
        }
    });
}
