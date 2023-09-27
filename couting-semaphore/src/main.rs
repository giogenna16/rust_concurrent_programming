use std::sync::Arc;
use std::thread;
use std::time::Duration;
use couting_semaphore::{CountingSemaphoreWithChannel, CountingSemaphoreWithCondvar};

fn main() {
    //let semaphore= Arc::new(CountingSemaphoreWithChannel::new(2));
    let semaphore= Arc::new(CountingSemaphoreWithCondvar::new(2));

    thread::scope(|s|{
        for i in 0..5{
            let semaphore_clone= semaphore.clone();
            s.spawn(move ||{
                println!("Thread {} is trying to acquire a resource", i);
                semaphore_clone.wait();
                println!("Thread {} acquired a resource", i);

                // simulate an operation
                thread::sleep(Duration::from_secs(10));

                println!("Thread {} release a resource", i);
                semaphore_clone.signal();
            });
        }
    });


}
