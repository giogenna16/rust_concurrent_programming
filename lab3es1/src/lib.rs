use itertools::Itertools;
use std::sync::{Arc, Mutex};
use std::{thread};

pub const NUMBERS: usize= 6;
const RESULT: f64 = 10.0;
const N_THREADS: usize = 4;

pub fn compute_all_couples(
    all_couples: &mut Vec<(Vec<char>, Vec<char>)>,
    numbers: &str,
    operators: &str,
) {
    for perm1 in numbers.chars().permutations(NUMBERS).unique() {
        for perm2 in operators
            .chars()
            .combinations_with_replacement(NUMBERS-1)
        {
            for perm3 in perm2.clone().into_iter().permutations(perm2.len()).unique() {
                all_couples.push((perm1.clone(), perm3.clone()));
            }
        }
    }
}

fn compute_valid_couples(
    all_couples: &Vec<(Vec<char>, Vec<char>)>,
    valid_couples: &mut Vec<String>,
) {
    for nums_ops in all_couples {
        let mut result = 0.0;
        for i in 0..nums_ops.0.len() - 1 {
            if i == 0 {
                result = extract_operators(
                    &nums_ops.1[i],
                    nums_ops.0[i].to_digit(10).unwrap() as f64,
                    nums_ops.0[i + 1].to_digit(10).unwrap() as f64,
                );
            } else {
                result = extract_operators(
                    &nums_ops.1[i],
                    result,
                    nums_ops.0[i + 1].to_digit(10).unwrap() as f64,
                );
            }
        }
        if result == RESULT {
            let mut result = "".to_string();
            for i in 0..nums_ops.0.len() {
                if i < nums_ops.0.len() - 1 {
                    result.push(nums_ops.0[i].clone());
                    result.push(nums_ops.1[i].clone());
                } else {
                    result.push(nums_ops.0[i].clone());
                }
            }
            valid_couples.push(result.clone());
        }
    }
}

fn extract_operators(op: &char, a: f64, b: f64) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => -1.0, // something went wrong
    }
}

pub fn compute_final_result_without_threads(all_couples: &Vec<(Vec<char>, Vec<char>)>) -> Vec<String>{
    let mut valid_couples: Vec<String> = Vec::new();
    compute_valid_couples(&all_couples, &mut valid_couples);
    /*println!("The solution (the sequences of elementary operations necessary to obtain {}) found is the following:", RESULT);
    for e in valid_couples {
        println!("{}", e)
    }*/
    return valid_couples;
}


pub fn compute_final_result_using_threads1(all_couples: &Vec<(Vec<char>, Vec<char>)>) -> Vec<String>{
    let cs: Vec<&[(Vec<char>, Vec<char>)]> = all_couples.chunks(all_couples.len()/N_THREADS).collect();
    let chunks= Arc::new(cs);
    let valid_couples  = Arc::new(Mutex::new(Vec::new()));
    thread::scope(|s|{
        for i in 0..N_THREADS{
            // The clone() method of an Arc  does a duplication of the ownership (of the pointer to
            // the block on the heap), taking care of increasing (in atomic way) the counter of the
            // references linked to the data
            let chunks= chunks.clone(); // Arc::clone(&chunks); is equivalent
            let valid_couples= valid_couples.clone();
            s.spawn(move ||{
                let mut valid_couples_thread= Vec::new();
                compute_valid_couples(& chunks[i].to_vec(), &mut valid_couples_thread);
                //compute_valid_couples(& chunks[i].to_vec(), &mut valid_couples.lock().unwrap());
                // I could call directly the commented function above to compute  the valid couples directly on the
                // threads shared Vec valid_couples; but it is not convenient because the mutual exclusion
                // acquisition of the Vec (valid_couples.lock().unwrap()) would last for as long as the function
                // is executing.
                // Instead the current thread does its computation saving the result in valid_couples_thread,
                // that is local within the thread and then extend the threads shared Vec valid_couples with
                // valid_couples_thread
                valid_couples.lock().unwrap().extend(valid_couples_thread);
            });
        }
    });
    /*println!("The solution (the sequences of elementary operations necessary to obtain {}) found is the following:", RESULT);
    for e in valid_couples.lock().unwrap().iter(){
        println!("{}", e)
    }*/
    return valid_couples.lock().unwrap().clone();
}

pub fn compute_final_result_using_threads2(all_couples: &Vec<(Vec<char>, Vec<char>)>) -> Vec<String>{
    let  iter= Arc::new(Mutex::new(all_couples.chunks(all_couples.len()/N_THREADS)));
    let  valid_couples = Arc::new(Mutex::new(Vec::new()));
    // Bad implementation because it uses Arc<Mutex<...>> both for all_couples and valid_couples, slowing down
    // the performance because each thead has to access this Vecs in mutual exclusion (lock) for practically all
    // the time the thread works, even if, as it is possible to see in previous implementation, it is not necessary.
    thread::scope(|s| {
         for _ in 0..N_THREADS {
             s.spawn( || {
                 compute_valid_couples(  & iter.lock().unwrap().next().unwrap().to_vec(), &mut valid_couples.lock().unwrap());
             });
         }
    });
    /*println!("The solution (all the sequences of elementary operations necessary to obtain {}) found is the following:", RESULT);
    for e in valid_couples.lock().unwrap().iter(){
         println!("{}", e)
     }*/
    return valid_couples.lock().unwrap().clone();
}

pub fn compute_final_result_using_threads_interleaved(all_couples: &Vec<(Vec<char>, Vec<char>)>) -> Vec<String> {
    let all_couples= Arc::new(all_couples);
    let valid_couples  = Arc::new(Mutex::new(Vec::new()));
    // The concurrency logic is equal to the fn compute_final_result_using_threads1. The change is the
    // division of all_couples between threads, that is done in interleaved way; that is to say, with three
    // threads: the first tries the permutations with index 0,3,6,..., the second 1,4,7,... and the third 2,5,8,...
    thread::scope(|s|{
        for i in 0..N_THREADS{
            // The clone() method of an Arc  does a duplication of the ownership (of the pointer to
            // the block on the heap), taking care of increasing (in atomic way) the counter of the
            // references linked to the data
            let all_couples=  Arc::clone(&all_couples);
            let valid_couples= Arc::clone(&valid_couples);
            s.spawn(move ||{
                let mut valid_couples_thread= Vec::new();
                for j  in 0..all_couples.len() {
                    if j % N_THREADS == i { // 'if condition' to do an interleaved division between threads
                        let mut result = 0.0;
                        for k in 0..all_couples[j].0.len() - 1 {
                            if k == 0 {
                                result = extract_operators(
                                    &all_couples[j].1[k],
                                    all_couples[j].0[k].to_digit(10).unwrap() as f64,
                                    all_couples[j].0[k + 1].to_digit(10).unwrap() as f64,
                                );
                            } else {
                                result = extract_operators(
                                    &all_couples[j].1[k],
                                    result,
                                    all_couples[j].0[k + 1].to_digit(10).unwrap() as f64,
                                );
                            }
                        }
                        if result == RESULT {
                            let mut result = "".to_string();
                            for k in 0..all_couples[j].0.len() {
                                if k < all_couples[j].0.len() - 1 {
                                    result.push(all_couples[j].0[k].clone());
                                    result.push(all_couples[j].1[k].clone());
                                } else {
                                    result.push(all_couples[j].0[k].clone());
                                }
                            }
                            valid_couples_thread.push(result.clone());
                        }
                    }
                }
                valid_couples.lock().unwrap().extend(valid_couples_thread);
            });
        }
    });
    //println!("The solution (the sequences of elementary operations necessary to obtain {}) found is the following:", RESULT);
    /*for e in valid_couples.lock().unwrap().iter(){
        println!("{}", e)
    }*/
    return valid_couples.lock().unwrap().clone();
}