use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use delayed_queue::DelayedQueue;

fn main() {
    let delayed_queue = Arc::new(DelayedQueue::new());

    let delayed_queue_clone = Arc::clone(&delayed_queue);

    delayed_queue_clone.offer(0, Instant::now() - Duration::from_secs(2));
    delayed_queue_clone.offer(1, Instant::now() + Duration::from_secs(2));
    delayed_queue_clone.offer(2, Instant::now() - Duration::from_secs(4));
    delayed_queue_clone.offer(3, Instant::now() + Duration::from_secs(4));
    delayed_queue_clone.offer(4, Instant::now() - Duration::from_secs(8));
    delayed_queue_clone.offer(5, Instant::now() + Duration::from_secs(8));
    delayed_queue_clone.offer(6, Instant::now() - Duration::from_secs(16));
    delayed_queue_clone.offer(7, Instant::now() + Duration::from_secs(16));

    thread::scope(|s|{
        let delayed_queue_thread1 = Arc::clone(&delayed_queue);
        s.spawn(move || {
            for _ in 0..8{
                let value= delayed_queue_thread1.take();
                if value.is_some(){
                    println!("The extracted value is: {}", value.unwrap());
                }else{
                    println!("The queue is empty!");
                }
            }
        });

        let delayed_queue_thread2 = Arc::clone(&delayed_queue);
        s.spawn(move || {
            for _ in 0..8{
                let len= delayed_queue_thread2.size();
                println!("The length is: {}", len);
            }
        });
    });
}
