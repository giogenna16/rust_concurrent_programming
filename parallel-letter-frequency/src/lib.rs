use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread};
use substring::Substring;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // First create a unique String from the input: &[&str]; then divide it in 'worker_count'
    // substrings/chunks of equal length, so that each thread operates on a different substring/chunk

    // letter_count must be an Arc<Mutex<...>> because it is accessed in read/write by 'worker_count' threads
    let letter_count= Arc::new(Mutex::new(HashMap::<char, usize>::new()));

    let mut input_string= "".to_string();
    for s in input{
        input_string.push_str(s);
    }
    let chunks= create_chunks(&input_string, worker_count);
    //arc_chunks should be an Arc<...> because it is accessed only for read by 'worker_count' threads
    let arc_chunks= Arc::new(chunks);

    thread::scope(|s| {
        for i in 0..worker_count {
            let chunks_clone= Arc::clone(&arc_chunks);
            let letter_count_clone= Arc::clone(&letter_count);
            s.spawn( move || {
                compute_letter_count(&chunks_clone[i], &mut letter_count_clone.lock().unwrap());
            });
        }
    });
    return letter_count.lock().unwrap().clone();
}


fn compute_letter_count(input: &str, result: &mut HashMap<char, usize>){
    for  c in input.chars(){
        // Numbers and punctuations do not count.
        // An uppercase letter and the same letter, but in lowercase, must be treated as equal
        // (counted as lowercase).
        if c.is_alphabetic(){
            result.entry(c.to_ascii_lowercase()).and_modify(|count| { *count += 1 }).or_insert(1);
        }
    }
}


fn create_chunks(input: &String, worker_count: usize)-> Vec<&str>{
    // create as many 'chunks' as there are workers
    let mut v = Vec::new();
    for i in 0..(worker_count){
        v.push(input.substring(i*input.len()/worker_count, (i+1)*input.len()/worker_count));
    }
    return v;
}

