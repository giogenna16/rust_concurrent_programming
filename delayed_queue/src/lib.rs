use std::sync::{Condvar, Mutex};
use std::time::Instant;

pub struct DelayedQueue<T: Send>{
    mutex: Mutex<Vec<(T, Instant)>>,
    cvar: Condvar
}

impl<T: Send> DelayedQueue<T> {

    pub fn new()-> Self{
        DelayedQueue{
            mutex: Mutex::new(Vec::new()),
            cvar: Condvar::new()
        }
    }

    pub fn offer(&self, t: T, i: Instant){
        let mut mutex = self.mutex.lock().unwrap();
        // Inserisco t e il suo Instant
        mutex.push((t, i));
        // Avviso che c'è stato un inserimento in coda
        self.cvar.notify_all();
    }

    pub fn take(&self)-> Option<T>{
        let mut mutex= self.mutex.lock().unwrap();
        loop {
            if mutex.len() == 0{
                // Se non c'è nessun elemento in coda, ritorno None
                return None;
            }

            let mut min = mutex[0].1;
            let mut index = 0;
            let now = Instant::now();

            for i in 1.. mutex.len() {
                let inst = mutex[i].1;
                if inst < min {
                    min = inst;
                    index = i;
                }
            }

            if min < now {
                // Istante passato, posso ritornare
                let t = mutex.remove(index).0;
                drop(mutex);
                self.cvar.notify_all();
                return Some(t);
            }

            // Istante futuro: attendo fino all'istante o all'arrivo di una notifica
            mutex= self.cvar
                .wait_timeout(mutex, min - now)
                .unwrap()
                .0;
        }
    }

    pub fn size(&self)-> usize{
        let mutex= self.mutex.lock().unwrap();
        return mutex.len();
    }
}

