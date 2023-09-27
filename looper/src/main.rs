use std::sync::Arc;
use std::thread;
use looper::{cleanup, Looper, Message, process};

fn main() {
    let looper= Arc::new(Looper::new(|msg|{process(msg)}, ||{cleanup()}));

    let looper_main= Arc::clone(&looper);
    looper_main.send(Message::new("Main message0"));

    thread::scope(|s|{
        let looper_clone0= Arc::clone(&looper);
        s.spawn(move ||{
            looper_clone0.send(Message::new("Thread0 message"));
        });

        let looper_clone1= Arc::clone(&looper);
        s.spawn(move ||{
            looper_clone1.send(Message::new("Thread1 message"));
        });

        let looper_clone2= Arc::clone(&looper);
        s.spawn(move ||{
            looper_clone2.send(Message::new("Thread2 message"));
        });
    });

    looper_main.send(Message::new("Main message1"));
}
