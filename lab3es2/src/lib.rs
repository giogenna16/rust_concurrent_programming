use std::sync::Mutex;
use rand::Rng;

pub const CIRCULAR_BUFFER_SIZE :u32 = 20;
pub const SENSOR_DATA_VALUES_LEN: usize = 10;

/**
    Circular buffer using Mutex
 **/
pub struct RingBuffer<T: Clone+Default> {
    mutex: Mutex<(Vec<T>, u32, u32, u32)> //buffer, read_index, write_index, free_cells
}

impl<T: Clone+Default> RingBuffer<T>{
    pub fn new()-> Self{
        RingBuffer{
            mutex: Mutex::new((vec![T::default(); CIRCULAR_BUFFER_SIZE as usize], 0, 0, CIRCULAR_BUFFER_SIZE)),
        }
    }

    pub fn read(&self)-> Option<T>{
        let mut mutex= self.mutex.lock().unwrap();
        return if mutex.3 == CIRCULAR_BUFFER_SIZE {
            None //the buffer is empty
        }else {
            let  value = mutex.0[mutex.1 as usize].clone();
            mutex.1 = (mutex.1 + 1) % CIRCULAR_BUFFER_SIZE;
            mutex.3 += 1;
            Some(value)
        }
    }

    pub fn write(&self, value: T)-> Result<(),()>{
        let mut mutex= self.mutex.lock().unwrap();
        return if mutex.3== 0{
            Err(()) //the buffer is full
        }else{
            let write_index= mutex.2;
            mutex.0[write_index as usize] = value;
            mutex.2 = (mutex.2 + 1) % CIRCULAR_BUFFER_SIZE;
            mutex.3 -= 1;
            Ok(())
        }
    }
}

pub fn generate_numbers()-> [f64; SENSOR_DATA_VALUES_LEN]{
    let mut values= [0.0; SENSOR_DATA_VALUES_LEN];
    for i in 0..SENSOR_DATA_VALUES_LEN{
        values[i]= rand::thread_rng().gen_range(-20..=40) as f64; // generate a random number from -20 to 40
    }
    return values;
}