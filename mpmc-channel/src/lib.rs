use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct MpMcChannelProf<E: Send>{
    size: usize,
    data: Mutex<(VecDeque<E>, bool)>,
    cv: Condvar,
}

impl<E: Send> MpMcChannelProf<E>{
    pub fn new(size: usize)-> Self{
        Self{
            size,
            data: Mutex::new((VecDeque::with_capacity(size), true)),
            cv: Condvar::new()
        }
    }

    pub fn recv(&self)-> Option<E> {
        let mut t= self.data.lock().ok()?;
        if !t.1 && t.0.is_empty(){
            return None;
        }
        t = self.cv.wait_while(t, |tuple|tuple.1==true && tuple.0.is_empty()).ok()?;
        if t.0.is_empty(){
            return None;
        }
        let e= t.0.pop_front();
        self.cv.notify_all();
        e
    }

    pub fn send(&self, e: E)->Option<()>{
        let mut t= self.data.lock().ok()?;
        if !t.1{
            return None;
        }
        t= self.cv.wait_while(t, |tuple| tuple.1== true && tuple.0.len()== self.size).ok()?;
        if !t.1{
            return None;
        }
        t.0.push_back(e);
        drop(t);
        self.cv.notify_all();
        Some(())
    }

    pub fn shutdown(&self)-> Option<()>{
        let mut t= self.data.lock().ok()?;
        t.1= false;
        self.cv.notify_all();
        Some(())
    }
}


pub struct MpMcChannelMine<E: Send+Clone+Default>{
    total_resources: usize,
    mutex: Mutex<(Vec<E>, usize, usize, usize, bool)>,
    condvar: Condvar,
}

impl <E: Send+Clone+Default> MpMcChannelMine<E>{
    pub fn new(n: usize)-> Self{
        MpMcChannelMine{
            total_resources: n,
            mutex: Mutex::new((vec![E::default(); n], 0, 0, n, true)), // buffer, read_index, write_index, free_cells, flag
            condvar: Condvar::new()
        }
    }

    pub fn send(&self, e: E)-> Option<()>{
        let mut mutex = self.mutex.lock().unwrap();

        if !mutex.4 {
            return None;
        }
        while mutex.3==0 && mutex.4 {
            mutex = self.condvar.wait(mutex).unwrap();
        }
        if !mutex.4 {
            return None;
        }

        let write_index= mutex.2;
        mutex.0[write_index]= e;
        mutex.2= (mutex.2 + 1) % self.total_resources;
        mutex.3-= 1;
        drop(mutex);

        self.condvar.notify_all();
        return Some(());
    }

    pub fn recv(&self)-> Option<E>{
        let mut mutex = self.mutex.lock().unwrap();

        if !mutex.4 && mutex.3== self.total_resources{
            return None;
        }
        while mutex.3== self.total_resources && mutex.4 {
            mutex = self.condvar.wait(mutex).unwrap();
        }
        if mutex.3== self.total_resources{
            return None;
        }

        let value=  mutex.0[mutex.1].clone();
        mutex.1= (mutex.1 + 1) % self.total_resources;
        mutex.3+= 1;
        drop(mutex);

        self.condvar.notify_all();
        Some(value)
    }

    pub fn shutdown(&self)-> Option<()>{
        let mut mutex = self.mutex.lock().unwrap();
        mutex.4= false;
        drop(mutex);
        self.condvar.notify_all();
        Some(())
    }
}
