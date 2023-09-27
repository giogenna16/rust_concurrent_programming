use std::sync::Arc;
use std::thread;
use challenge::Challenge;
use rand::Rng;

fn main() {
    let challenge=  Arc::new(Challenge::new());

    thread::scope(|s|{
        let challenge_1= challenge.clone();
        s.spawn(move ||{
            loop{
                challenge_1.result1(rand::thread_rng().gen_range(0..10)).expect("Can be called only once");
            }
        });

        let challenge_2= challenge.clone();
        s.spawn(move||{
            loop{
                challenge_2.accept(rand::thread_rng().gen_bool(0.8)).expect("Can be called only once");
                challenge_2.result2(rand::thread_rng().gen_range(0..10)).expect("Can be called only once");
            }
        });

        let challenge_judge= challenge.clone();
        s.spawn(move ||{
           loop{
                let result= challenge_judge.winner();
                if result.is_some(){
                    println!("Result:  {}", result.unwrap());
                }
           }
        });
    });
}
