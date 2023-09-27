use std::sync::Arc;
use std::thread;
use cache::Cache;

fn main() {
    let cache= Arc::new(Cache::new());

    thread::scope(|s|{
        for i in 0..5{
            let cache_clone= cache.clone();
            if i%2 == 0{
                s.spawn(move||{
                    let r= cache_clone.get(i, |i|{i+1});
                    println!("Thread {}, result: {}", i, r);
                });
            }else{
                s.spawn(move||{
                    let r= cache_clone.get(i+1, |i|{i+1});
                    println!("Thread {}, result: {}", i, r);
                });
            }
        }
    });
}
