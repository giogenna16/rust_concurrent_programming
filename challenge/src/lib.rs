use std::sync::{Condvar, Mutex};

pub struct Challenge{
    mutex: Mutex<(bool, usize, Option<bool>, Option<isize>, Option<isize>)>,
    cvar: Condvar
}

impl Challenge{
    pub fn new()-> Self{
        Challenge{
            mutex: Mutex::new((false, 0, None, None, None)), // isStarted, isFinished (if it==2), isAccepted, score1, score2
            cvar: Condvar::new()
        }
    }

    pub fn accept(&self, response: bool)-> Result<(), ()>{
        let mut mutex= self.mutex.lock().unwrap();
        // if it is started, wait
        while mutex.0 {
            mutex= self.cvar.wait(mutex).unwrap();
        }
        // it can be called only once (per match)
        return if mutex.2.is_some() {
            Err(())
        } else {
            mutex.0 = true; // started
            mutex.2 = Some(response);
            self.cvar.notify_all();
            Ok(())
        }
    }

    pub fn result1(&self, score: isize)-> Result<(), ()>{
        let mut mutex= self.mutex.lock().unwrap();
        // If it is not started or it is finished or it already wrote its result, wait
        while !mutex.0 || mutex.1==2 || (mutex.1==1 && mutex.3.is_some()) {
            mutex= self.cvar.wait(mutex).unwrap();
        }

        // It can be called only once (per match)
        return if mutex.3.is_some() {
            Err(()) //It should not ever enter here
        }else{
            mutex.1+= 1; // finished
            mutex.3 = Some(score);
            self.cvar.notify_all();
            Ok(())
        }
    }

    pub fn result2(&self, score: isize)-> Result<(), ()>{
        let mut mutex= self.mutex.lock().unwrap();
        // If it is not started or it is  finished or it already wrote its result, wait
        while !mutex.0 || mutex.1==2 || (mutex.1==1 && mutex.4.is_some()) {
            mutex = self.cvar.wait(mutex).unwrap();
        }

        // It can be called only once (per match)
        return if mutex.4.is_some() {
            Err(()) //It should not ever enter here
        }else{
            mutex.1+= 1; // finished
            mutex.4= Some(score);
            self.cvar.notify_all();
            Ok(())
        }
    }

    pub fn winner(&self)-> Option<isize>{
        let mut mutex= self.mutex.lock().unwrap();
        // If it is not started or (it is started and it is not finished), wait
        while !mutex.0 || (mutex.0 && mutex.1 != 2) {
            mutex= self.cvar.wait(mutex).unwrap();
        }

        let result;
        if mutex.2 == Some(false) {
            result= -1; // not accepted
        } else {
            let diff = mutex.3.unwrap() - mutex.4.unwrap();
            if diff == 0 {
                result= 0; // tied
            } else if diff >  0 {
                result= 1; // 1 won
            } else {
                result= 2; // 2 won
            }
        }
        // reset all
        mutex.0= false; // not started
        mutex.1= 0; // not finished
        mutex.2= None;
        mutex.3= None;
        mutex.4= None;
        self.cvar.notify_all();
        return Some(result);
    }
}