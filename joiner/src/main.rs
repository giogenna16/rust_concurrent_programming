use std::sync::Arc;
use std::thread;
use joiner::{Joiner, JoinerCvar, JoinerWithConsumer};

fn main() {

    let joiner= Arc::new(Joiner::new(4));

    thread::scope(|s|{
        for i in 0..4{
            let joiner_clone= joiner.clone();
            s.spawn(move ||{
                for j in 0..3{
                    let result= joiner_clone.supply(i, j as f64);
                    println!("{:?}", result);
                }
            });
        }
    });

    println!("-------------------------------");

    /**********************************************************************************************/

    let joiner= Arc::new(JoinerCvar::new(4));

    thread::scope(|s|{
        for i in 0..4{
            let joiner_clone= joiner.clone();
            s.spawn(move ||{
                for j in 0..3{
                    let result= joiner_clone.supply(i, j as f64);
                    println!("{:?}", result);
                }
            });
        }
    });

    println!("-------------------------------");

    /**********************************************************************************************/

    let joiner= Arc::new(JoinerWithConsumer::new(4));

    thread::scope(|s|{
        for i in 0..4{
            let joiner_clone= joiner.clone();
            s.spawn(move ||{
                for j in 0..3{
                    let m= joiner_clone.supply(i, j as f64);
                    println!("{:?}", m);
                }
            });
        }
    });
}
