use std::sync::Arc;
use std::thread;
use std::time::Duration;
use synchronizer::Synchronizer;

fn main() {

    let synchronizer= Arc::new(Synchronizer::new(|a, b|{a + b}));

    thread::scope(|s|{
        let synchronizer1= synchronizer.clone();
        s.spawn(move ||{
            for i in 0..5{
                thread::sleep(Duration::from_secs(i+1));
                synchronizer1.data_from_first_port(0.1* (i as f64));
            }
        });

        let synchronizer2= synchronizer.clone();
        s.spawn(move ||{
            for i in 0..5{
                thread::sleep(Duration::from_secs(i));
                synchronizer2.data_from_second_port(0.2* (i as f64));
            }
        });
    });
}
