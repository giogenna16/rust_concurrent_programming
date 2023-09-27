use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Condvar, Mutex};

pub struct RankingBarrierChannel{
    n_threads: usize,
    senders: Mutex<Vec<Sender<()>>>,
    receivers: Vec<Mutex<Receiver<()>>>,
    ranking: Mutex<usize>
}

impl RankingBarrierChannel{
    pub fn new(n_threads: usize)-> Result<Self, String>{
        if n_threads < 2{
            Err(String::from("There must be at least 2 threads!"))
        }else{
            let (mut senders, mut receivers)= (Vec::new(), Vec::new());
            for _ in 0..n_threads{
                let (sender, receiver) = channel();
                senders.push(sender);
                receivers.push(Mutex::new(receiver));
            }

            Ok(RankingBarrierChannel{
                n_threads,
                senders: Mutex::new(senders),
                receivers,
                ranking: Mutex::new(1)
            })
        }
    }

    pub fn wait(&self, i: usize)-> usize{
        let mut ranking= self.ranking.lock().unwrap();
        let local_ranking= *ranking;
        if *ranking == self.n_threads{
            *ranking = 1;
        }else{
            *ranking+= 1;
        }
        drop(ranking);

        for j in 0..self.n_threads{
            let senders= self.senders.lock().unwrap();
            if i != j{
                senders[j].send(()).unwrap();
            }
            drop(senders);
        }

        for _ in 0..(self.n_threads-1){
            self.receivers[i].lock().unwrap().recv().unwrap();
        }

        return local_ranking
    }
}


/**************************************************************************************************/


pub struct RankingBarrierCvar{
    n_threads: usize,
    mutex: Mutex<(usize, usize)>, //count, generation
    condvar: Condvar,
}

impl RankingBarrierCvar{
    pub fn new(n_threads: usize)-> Result<Self, String>{
        if n_threads < 2{
            Err(String::from("There must be at least 2 threads!"))
        }else{
            Ok(RankingBarrierCvar{
                n_threads,
                mutex: Mutex::new((0, 0)),
                condvar: Condvar::new()
            })
        }
    }

    pub fn wait(&self)-> usize{
        let mut mutex = self.mutex.lock().unwrap();
        let local_generation= mutex.1;
        mutex.0+= 1;
        let my_ranking= mutex.0;

        if mutex.0 < self.n_threads{
            while mutex.1 == local_generation{
                mutex= self.condvar.wait(mutex).unwrap();
            }
        }else{
            mutex.0= 0;
            mutex.1+= 1;
            drop(mutex);
            self.condvar.notify_all();
        }
        return my_ranking
    }
}