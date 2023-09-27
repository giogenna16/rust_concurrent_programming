use std::sync::{Condvar, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

/** Cyclic barrier implemented using Mutex<...>, Condvar **/
pub struct CyclicBarrierMC{
    n_threads: usize,
    mutex: Mutex<(usize, usize)>,
    condvar: Condvar
}

impl CyclicBarrierMC{
    pub fn new(n_threads: usize)-> Self{
        CyclicBarrierMC{
            n_threads,
            mutex: Mutex::new((0, 0)),
            condvar: Condvar::new()

        }
    }

    pub fn wait(&self){
        let mut mutex= self.mutex.lock().unwrap();
        let local_generation= mutex.1;
        mutex.0+= 1;

        if mutex.0 < self.n_threads{
            while local_generation ==  mutex.1{
               mutex= self.condvar.wait(mutex).unwrap();
            }
        }else{
            mutex.0 = 0;
            mutex.1 += 1;
            self.condvar.notify_all();
        }
    }
}

/** Cyclic barrier implemented using channel(). The idea is that each thread at the entrance sends a
message to all the other threads and waits until it receives all the messages from the other threads */
pub struct CyclicBarrierC {
    n_threads: usize,
    senders: Mutex<Vec<Sender<()>>>,
    receivers: Vec<Mutex<Receiver<()>>>,
}

impl CyclicBarrierC{

    pub fn new(n_threads: usize) -> Self {
        let (mut senders, mut receivers) = (Vec::new(), Vec::new());
        for _ in 0..n_threads {
            let (sender, receiver) = channel(); // One channel per thread
            let receiver=  Mutex::new(receiver);
            senders.push(sender);
            receivers.push(receiver);
        }
        CyclicBarrierC {
            n_threads,
            senders: Mutex::new(senders),
            receivers
        }
    }

    pub fn wait(&self, i: usize) {
        for j in 0..(self.n_threads){
            let senders= self.senders.lock().unwrap();
            if i != j{
                senders[j].send(()).unwrap(); // Thread i sends message to thread j
            }
            drop(senders);
        }

        for _ in 0..(self.n_threads-1){
            self.receivers[i].lock().unwrap().recv().unwrap(); // Thread i waits for N-1 messages (threads), before proceeding
        }
    }
}


/** Cyclic barrier implemented using channel(): The idea is to have synchronization managed by an
additional thread that acts as a "coordination thread" (it decides whether threads can go forward);
each "worker" thread sends a message to the coordinator and waits until it receives a "response"
message  from the coordinator*/
pub struct CyclicBarrierCCord {
    receivers: Vec<Mutex<Receiver<()>>>,
    coordinator_sender: Mutex<Sender<()>>,
}

impl  CyclicBarrierCCord {

    pub fn new( n_threads: usize) -> Self{
        let (mut senders, mut receivers) = (Vec::new(), Vec::new());
        for _ in 0..n_threads {
            let (sender, receiver) = channel(); // One channel per thread
            let receiver=Mutex::new(receiver);
            senders.push(sender);
            receivers.push(receiver);
        }

        let (coordinator_sender, coordinator_receiver)= channel::<()>(); //one plus channel for the coordination thread
        // Spawn the coordination thread with an infinite loop inside because it has to live until the
        // struct CyclicBarrierCCord is alive: it will be automatically dropped when the CyclicBarrierCCord
        // struct is dropped.
        thread::spawn(move || {
            loop {
                // Coordination thread  waits for N messages (threads)
                for _ in 0..n_threads{
                    if coordinator_receiver.recv().is_err(){
                        return;
                    }
                }
                // Coordination thread sends response to the N threads
                for j in 0..n_threads{
                    if senders[j].send(()).is_err(){
                        return;
                    }
                }
            }
        });

        CyclicBarrierCCord{
            receivers,
            coordinator_sender: Mutex::new(coordinator_sender)
        }
    }

    pub fn wait(&self, i: usize){
        // Thread i sends a message to the coordination thread
        self.coordinator_sender.lock().unwrap().send(()).unwrap();

        // Thread i waits for the response from the coordination thread
        self.receivers[i].lock().unwrap().recv().unwrap();
    }
}
