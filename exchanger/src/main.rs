use std::sync::Arc;
use std::thread;
use exchanger::{Exchanger, ExchangerChannel};

fn main() {
    let exchanger= Arc::new(Exchanger::new());

    thread::scope(|s| {
        //Thread0
        let exchanger_t0= exchanger.clone();
        s.spawn(move ||{
            loop{
                //let message = exchanger_t0.exchange(0, "Hi thread1, I am thread0"); //if use channel
                let message = exchanger_t0.exchange( 0, "Hi thread1, I am thread0");
                println!("Thread0 received: '{}'", message);
            }
        });

        //Thread1
        let exchanger_t1= exchanger.clone();
        s.spawn(move ||{
            loop{
                //let message= exchanger_t1.exchange( 1, "Hi thread0, I am thread1"); //if use channel
                let message= exchanger_t1.exchange( 1, "Hi thread0, I am thread1");
                println!("Thread1 received: '{}'", message);
            }
        });
    });
}
