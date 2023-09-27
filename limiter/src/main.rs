use std::sync::Arc;
use std::thread;
use limiter::{Limiter, LimiterChannel};

fn main() {
   let limiter= Arc::new(Limiter::new(2));

    thread::scope(|s|{
        for i in 0..5{
            let limiter_clone= Arc::clone(&limiter);
            s.spawn(move ||{
                limiter_clone.wait_and_signal(i);
            });
        }
    });
}
