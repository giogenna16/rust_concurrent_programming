use std::collections::HashMap;
use std::hash::Hash;
use std::io::ErrorKind;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub struct Cache<K: Clone + Eq + Hash, V: Clone>{
    map: Mutex<HashMap<K, V>>,
}

impl<K: Clone + Eq + Hash, V: Clone> Cache<K, V>{
    pub fn new()->Self{
        Cache{
            map: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, k : K, f: impl FnOnce(&K)->V+'static)-> Arc<V>{
        let mut map = self.map.lock().unwrap();
        if map.get(&k).is_none() {
            map.insert(k.clone(), f(&k.clone()));
        }
        return Arc::new( map.get(&k).unwrap().clone());
    }
}