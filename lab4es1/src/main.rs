use std::sync::{Arc};
use std::thread;
use lab4es1::{CyclicBarrierC, CyclicBarrierCCord, CyclicBarrierMC};

fn main() {
    let barrier = Arc::new(CyclicBarrierCCord::new(4));
    let mut vt = Vec::new();
    for i in 0..4 {
        let barrier = barrier.clone();
        vt.push(thread::spawn(move || {
            for j in 0..10 {
                //barrier.wait(); //if you use CyclicBarrierMC
                barrier.wait(i); // if you use CyclicBarrierC, CyclicBarrierCCord
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}

