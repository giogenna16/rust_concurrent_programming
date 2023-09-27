use std::sync::{Condvar, Mutex, MutexGuard};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
use rand::Rng;

pub const PRODUCERS_THREADS: usize= 10;

pub fn random_sleep_and_get(waiting_factor: f64, circular_buffer: &CircularBuffer, index: usize)  {
    // Generate a random number r, with 0 <= r <= 10
    let r = rand::thread_rng().gen_range(-0..=10) as u32;
    // Put the current thread to sleep for a random time depending on waiting_factor and r.
    thread::sleep(Duration::from_millis((300.0 * waiting_factor * (r as f64)) as u64));
    println!("Producer {} produces the value: {}", index, r);
    circular_buffer.write(r).expect("Producer: The buffer is full! CANNOT write!");
}

/**************************************************************************************************/

pub fn set_speed(mut waiting_factor: MutexGuard<f64>, circular_buffer: &CircularBuffer){
    let mut sum= 0;
    for _ in 0..PRODUCERS_THREADS {
        sum+= circular_buffer.read().expect("Consumer: The buffer is empty! CANNOT read!");
    }
    if sum > 50{
        // Increase the waiting factor: decelerate of 10%
        *waiting_factor = *waiting_factor * 1.1;
        println!("The consumer computes a sum equal to {}: greater than 50, so the waiting factor increases to: {}", sum, *waiting_factor);
    }else{
        // Decrease the waiting factor: accelerate of 10%
        *waiting_factor= *waiting_factor / 1.1;
        println!("The consumer computes a sum equal to {}: less than 50, so the waiting factor decreases to: {}", sum, *waiting_factor);
    }

}


/**************************************************************************************************/


/**
    Circular buffer using Mutex
 **/
pub struct CircularBuffer{
    mutex: Mutex<(Vec<u32>, u32, u32, u32)>, // buffer, read_index, write_index, free_cells
}

impl CircularBuffer{
    pub fn new()-> Self{
        CircularBuffer{
            mutex: Mutex::new((vec![u32::default(); PRODUCERS_THREADS], 0, 0, PRODUCERS_THREADS as u32))
        }
    }

    pub fn read(&self)-> Option<u32>{
        let mut mutex= self.mutex.lock().unwrap();

        return if mutex.3 == PRODUCERS_THREADS as u32 {
            None //the buffer is empty
        }else {
            let value= mutex.0[mutex.1 as usize];
            mutex.1= (mutex.1 + 1) % (PRODUCERS_THREADS as u32);
            mutex.3+= 1;
            Some(value)
        }
    }

    pub fn write(&self, value: u32)-> Result<(),()>{
        let mut mutex= self.mutex.lock().unwrap();

        return if mutex.3== 0{
            Err(()) //the buffer is full
        }else{
            let write_index= mutex.2;
            mutex.0[write_index as usize]= value;
            mutex.2= (mutex.2 + 1) % (PRODUCERS_THREADS as u32);
            mutex.3-= 1;
            Ok(())
        }
    }
}


/**************************************************************************************************/

/**
    A particular Cyclic Barrier implemented using channels to obtain the following behaviour: there
    are N producers threads which act in parallel: each thread produces a value and sends a message
    to the consumer (another thread, that, in the meanwhile, is blocked), then blocks itself until
    the response from the consumer arrives. The consumer, when receives all the N messages from the
    producers, is unblocked and consumes the N produced values; then it sends a message to each of
    the N producers to response and unblocks them.
**/
pub struct CyclicBarrierProducersConsumerCh{
    producers_senders: Vec<Mutex<Sender<()>>>,
    producers_receivers: Vec<Mutex<Receiver<()>>>,
    consumer_sender: Mutex<Sender<()>>,
    consumer_receiver: Mutex<Receiver<()>>,
}

impl CyclicBarrierProducersConsumerCh{
    pub fn new()-> Self{
        let (mut producers_senders, mut producers_receivers) = (Vec::new(), Vec::new());
        for _ in 0..PRODUCERS_THREADS {
            let (sender, receiver) = channel(); // One channel per thread
            let sender=  Mutex::new(sender);
            let receiver=  Mutex::new(receiver);
            producers_senders.push(sender);
            producers_receivers.push(receiver);
        }
        let (consumer_sender, consumer_receiver)= channel();
        CyclicBarrierProducersConsumerCh{
            producers_senders,
            producers_receivers,
            consumer_sender: Mutex::new(consumer_sender),
            consumer_receiver: Mutex::new(consumer_receiver)
        }
    }

    pub fn wait_for_consumer(&self, i: usize){
        // Producer thread i sends a message to the consumer thread
        self.consumer_sender.lock().unwrap().clone().send(()).unwrap();

        // Producer thread i waits for the response from the consumer thread
        self.producers_receivers[i].lock().unwrap().recv().unwrap();
    }

    pub fn wait_for_producers(&self){
        // Consumer thread  waits for N messages (threads)
        for _ in 0..PRODUCERS_THREADS {
            self.consumer_receiver.lock().unwrap().recv().unwrap();
        }
    }

    pub fn restart_producers(&self){
        // Consumer thread sends response to the N producers threads
        for j in 0..PRODUCERS_THREADS {
            self.producers_senders[j].lock().unwrap().send(()).unwrap();
        }
    }
}


/**************************************************************************************************/

/**
    A particular Cyclic Barrier implemented using Mutex<...> and Condvar to obtain the
    following behaviour: there are N producers threads which act in parallel: each thread produces a
    value and sends a message to the consumer (another thread, that, in the meanwhile, is blocked),
    then blocks itself until the response from the consumer arrives. The consumer, when receives all
    the N messages from the producers, is unblocked and consumes the N produced values; then it sends
    a message to each of the N producers to response and unblocks them.
**/

pub struct CyclicBarrierProducersConsumerCV {
    mutex: Mutex<(Vec<bool>, usize)>,
    condvar: Condvar
}

impl CyclicBarrierProducersConsumerCV {
    pub fn new() -> Self {
        CyclicBarrierProducersConsumerCV {
            mutex: Mutex::new((vec![false; PRODUCERS_THREADS], 0)),
            condvar: Condvar::new()
        }
    }

    pub fn wait_for_consumer(&self, i: usize) {
        let mut mutex= self.mutex.lock().unwrap();
        /* If 'count' reaches the number of producer threads, it notifies the condvar to
           wake up one (the consumer thread). */
        mutex.1+= 1;
        if mutex.1== PRODUCERS_THREADS{
            self.condvar.notify_one();
        }

        /* It locks the mutex and waits on the condvar until the
           flag becomes true, so until the consumer unblock the producers */
        while !mutex.0[i] {
            mutex = self.condvar.wait(mutex).unwrap();
        }
        mutex.0[i]= false; // once unblocked, "reset" to false
    }

    pub fn wait_for_producers(&self) {
        let mut mutex= self.mutex.lock().unwrap();
        /* It locks the  mutex and waits until 'count' reaches the
           number of producers threads */
        while mutex.1 < PRODUCERS_THREADS {
            mutex = self.condvar.wait(mutex).unwrap();
        }
        mutex.1= 0; //Once unblocked, reset to zero
    }

    pub fn restart_producers(&self) {
        /* It locks the mutex, sets flag to true, and notifies the  condvar to wake all*/
        let mut mutex= self.mutex.lock().unwrap();
        for i in 0..PRODUCERS_THREADS{
            mutex.0[i]= true;
        }
        self.condvar.notify_all();
    }
}
