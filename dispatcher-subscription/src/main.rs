use std::sync::Arc;
use std::thread;
use dispatcher_subscription::Dispatcher;

fn main() {
    let dispatcher= Arc::new(Dispatcher::new());

    let mut vt = Vec::new();
    for i in 0..5{
        let dispatcher_clone= Arc::clone(&dispatcher);
        if i%2 == 0{
            vt.push(thread::spawn(move ||{
                let subscriber= dispatcher_clone.subscribe();
                let message= subscriber.read();
                println!("Thread {} received '{}'", i, message.unwrap());
            }))
        }else{
            vt.push(thread::spawn(move ||{
                dispatcher_clone.dispatch(format!("Hi, I am thread {}", i));
            }))
        }
    }

    for v in vt{
        v.join().unwrap();
    }
}
