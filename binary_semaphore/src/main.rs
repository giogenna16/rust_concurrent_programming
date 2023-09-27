use std::sync::Arc;
use std::thread;
use std::time::Duration;
use binary_semaphore::{BinarySemaphore, BinarySemaphoreChannel};

fn main() {
    //let binary_semaphore= Arc::new(BinarySemaphore::new());
    let binary_semaphore= Arc::new(BinarySemaphoreChannel::new());

    thread::scope(|s|{

        // thread 0
        let binary_semaphore_clone0= Arc::clone(&binary_semaphore);
        s.spawn(move ||{
            loop{
                println!("Thread 0 is trying to acquire the resource");
                binary_semaphore_clone0.wait();
                println!("Thread 0 acquired the resource");
                thread::sleep(Duration::from_secs(3));
                binary_semaphore_clone0.signal();
                println!("Thread 0 released the resource");
            }
       });

        // thread 1
        let binary_semaphore_clone1= Arc::clone(&binary_semaphore);
        s.spawn(move ||{
            loop{
                println!("Thread 1 is trying to acquire the resource");
                binary_semaphore_clone1.wait();
                println!("Thread 1 acquired the resource");
                thread::sleep(Duration::from_secs(10));
                binary_semaphore_clone1.signal();
                println!("Thread 1 released the resource");
            }
        });
    });
}
