use std::sync::{Arc, Mutex};
use std::thread;
use sensors::{CircularBuffer, CyclicBarrierProducersConsumerCh, CyclicBarrierProducersConsumerCV, PRODUCERS_THREADS, random_sleep_and_get, set_speed};
/*
    A program has to monitor a machine by reading the values from 10 sensors, which take different
    times to provide the result and are read by 10 threads, one per sensor (simulate the reading with
    a random_sleep_and_get() function which does a random length sleep and returns a random number
    between 0 and 10). Once the 10 values have been collected, another thread collects the results and
    adds them; if the result is greater than 50 it slows down the car, if less than 50 it accelerates it
    (to be simulated with a set_speed() function which makes a sleep of random length). It is important
    that parameter reading (read cycle) and machine setting (write cycle) do not overlap, as the values
    could be perturbed. The program also does infinite read/write.
 */

fn main() {

    let waiting_factor:Arc<Mutex<f64>>= Arc::new(Mutex::new(1.0));
    let circular_buffer= Arc::new(CircularBuffer::new());
    //let cyclic_barrier= Arc::new(CyclicBarrierProducersConsumerCh::new());
    let cyclic_barrier= Arc::new(CyclicBarrierProducersConsumerCV::new());

    thread::scope(|s|{

        // producers
        for i in 0..PRODUCERS_THREADS{
            let waiting_factor_producer = Arc::clone(&waiting_factor);
            let circular_buffer_producer= Arc::clone(&circular_buffer);
            let cyclic_barrier_producer = Arc::clone(&cyclic_barrier);
            s.spawn(move ||{
                loop{
                    random_sleep_and_get(
                        waiting_factor_producer.lock().unwrap().clone(),
                        &circular_buffer_producer,
                        i
                    );

                    cyclic_barrier_producer.wait_for_consumer(i);
                }
            });
        }

        // consumer
        let waiting_factor_consumer = Arc::clone(&waiting_factor);
        let circular_buffer_consumer = Arc::clone(&circular_buffer);
        let cyclic_barrier_consumer = Arc::clone(&cyclic_barrier);
        s.spawn(move ||{
            loop{
                cyclic_barrier_consumer.wait_for_producers();

                set_speed(waiting_factor_consumer.lock().unwrap(), &circular_buffer_consumer);

                cyclic_barrier_consumer.restart_producers();
            }
        });
    });
}
