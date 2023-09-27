use std::sync::Arc;
use std::thread;
use ranking_barrier::{RankingBarrierChannel, RankingBarrierCvar};

fn main() {

    let ranking_barrier= RankingBarrierChannel::new(4);
    //let ranking_barrier= RankingBarrierCvar::new(4);

    if ranking_barrier.is_ok(){
        let ranking_barrier= Arc::new(ranking_barrier.unwrap());
        thread::scope(|s|{
            for i in 0..4{
                let ranking_barrier_clone= ranking_barrier.clone();
                s.spawn(move ||{
                    for j in 0..6{
                        let rank = ranking_barrier_clone.wait(i); // if RankingBarrierChannel
                        //let rank = ranking_barrier_clone.wait(); // if RankingBarrierCvar
                        println!("Round {}, Thread {}, Position {}", j, i, rank);
                    }
                });
            }
        });
    }else{
        println!("{}", ranking_barrier.err().unwrap());
    }
}
