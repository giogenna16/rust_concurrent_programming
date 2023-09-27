use std::sync::Arc;
use std::thread;
use std::time::Duration;
use count_down_latch::CountDownLatch;

fn main() {
    let count_down_latch= Arc::new(CountDownLatch::new(5));

    thread::scope(|s|{
        let count_down_latch_1= count_down_latch.clone();
        s.spawn(move ||{
            println!("Thread 1: await on the condition variable");
            count_down_latch_1.a_wait();
            println!("Await finished");
        });

        let count_down_latch_2= count_down_latch.clone();
        s.spawn(move ||{
            count_down_latch_2.count_down();
            thread::sleep(Duration::from_secs(3));
            count_down_latch_2.count_down();
            thread::sleep(Duration::from_secs(3));
            count_down_latch_2.count_down();
        });

        let count_down_latch_3= count_down_latch.clone();
        s.spawn(move ||{
            count_down_latch_3.count_down();
            thread::sleep(Duration::from_secs(3));
            count_down_latch_3.count_down();
            thread::sleep(Duration::from_secs(3));
            count_down_latch_3.count_down();
        });
    });
}
