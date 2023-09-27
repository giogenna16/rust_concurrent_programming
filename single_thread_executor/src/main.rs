use std::sync::Arc;
use std::thread;
use single_thread_executor::SingleThreadExecutor;

fn main() {
    let executor= Arc::new(SingleThreadExecutor::new());

    let executor_main= executor.clone();
    executor_main.submit(Box::new(move || {
        // random function
        let mut j = 0;
        for i in 0..10 {
            j += i * i % (i + j + 1);
        }
        println!("Main thread, first call: {}", j);
    }));

    executor_main.submit(Box::new(move || {
        // random function
        let mut j = 1;
        for i in 0..20 {
            j += i * i % (i + j);
        }
        println!("Main thread, second call: {}", j);
    }));


    thread::scope(|s|{
        let executor_t1= executor.clone();
        s.spawn(move ||{

            executor_t1.submit(Box::new(move || {
                // random function
                let mut j = 2;
                for i in 0..30 {
                    j += i * i % (i + j);
                }
                println!("Thread 1, first call: {}", j);
            }));

            executor_t1.join();
        });

        let executor_t2= executor.clone();
        s.spawn(move ||{
           executor_t2.submit(Box::new(move || {
                // random function
                let mut j = 3;
                for i in 0..40 {
                    j += i * i % (i + j);
                }
                println!("Thread 2, first call: {}", j);
            }));

            executor_t2.submit(Box::new(move || {
                // random function
                let mut j = 4;
                for i in 0..50 {
                    j += i * i % (i + j);
                }
                println!("Thread 2, second call: {}", j);
            }));
        });

        let executor_t3= executor.clone();
        s.spawn(move ||{
            executor_t3.close();
        });
    });
}
