use std::cell::RefCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use lab3es2::{RingBuffer, generate_numbers, SENSOR_DATA_VALUES_LEN};

fn main() {
    let ring_buffer= Arc::new(RingBuffer::<[f64; SENSOR_DATA_VALUES_LEN ]>::new());

    thread::scope(|s|{
        let ring_buffer_producer= Arc::clone(&ring_buffer);
        s.spawn(move ||{
            loop{
                let value= generate_numbers();
                println!("Producer: Acquired Data: {:?}", value);
                ring_buffer_producer.write(value).expect("Producer: The buffer is full! CANNOT write!");

                thread::sleep(Duration::from_millis(1000)); // 1s
            }
        });

        let ring_buffer_consumer= Arc::clone(&ring_buffer);
        s.spawn(move ||{
            thread::sleep(Duration::from_millis(10000)); //To give the time to the
            // producer to write the first values
            loop{
                println!("Consumer: Statistics of the acquired data:");
                for _ in 0..SENSOR_DATA_VALUES_LEN{
                    let value= ring_buffer_consumer.read().expect("Consumer: The buffer is empty! CANNOT read!");
                    let mut max= f64::MIN;
                    let mut min= f64::MAX;
                    let mut avg=  0.0;

                    for v in value{
                        if v > max {
                            max = v
                        }
                    }
                    for v in value{
                        if v < min {
                            min = v
                        }
                    }
                    for v in value{
                        avg+= v;
                    }
                    avg= avg/(SENSOR_DATA_VALUES_LEN as f64);
                    println!("\tMin Value: {}", min);
                    println!("\tMax Value: {}", max);
                    println!("\tAverage Value: {}\n", avg);
                }

                thread::sleep(Duration::from_millis(10000)); // 10s
            }
        });
    });
}
