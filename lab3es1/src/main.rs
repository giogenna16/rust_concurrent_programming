use std::io::stdin;
use std::time::Instant;
use lab3es1::{compute_all_couples, compute_final_result_without_threads, compute_final_result_using_threads2, compute_final_result_using_threads1, NUMBERS, compute_final_result_using_threads_interleaved};

fn main() {
    let operators = "+-*/";
    let mut numbers = String::new();

    println!("Enter {} numbers between 0 and 9 separated by a space:", NUMBERS);

    match stdin().read_line(&mut numbers).ok() {
        Some(_) => {
            let numbers: Vec<&str> = numbers.trim().split(" ").collect();
            let mut numbers_string = "".to_string();
            if numbers.len() != NUMBERS {
                println!("Invalid input!");
                return;
            }
            for n in numbers {
                if n.len() > 1 {
                    println!("Invalid input!");
                    return;
                }
                let n_char = n.chars().next().unwrap();
                if n_char.to_digit(10).is_some() {
                    numbers_string.push(n_char.clone());
                } else {
                    println!("Invalid input!");
                    return;
                }
            }

            let mut all_couples: Vec<(Vec<char>, Vec<char>)> = Vec::new();
            compute_all_couples(&mut all_couples, numbers_string.as_str(), operators);

            let before1 = Instant::now();
            let result1= compute_final_result_without_threads(& all_couples);
            println!("Elapsed time (without threads): {:?}; (found {} sequences).", before1.elapsed(), result1.len());

            let before2 = Instant::now();
            let result2= compute_final_result_using_threads1(& all_couples);
            println!("Elapsed time (with threads 'good'): {:?}; (found {} sequences).", before2.elapsed(), result2.len());

            let before3 = Instant::now();
            let result3= compute_final_result_using_threads2(& all_couples);
            println!("Elapsed time (with threads 'bad'): {:?}; (found {} sequences).", before3.elapsed(), result3.len());

            let before4 = Instant::now();
            let result4= compute_final_result_using_threads_interleaved(& all_couples);
            println!("Elapsed time (with threads 'good' interleaved): {:?}; (found {} sequences).", before4.elapsed(), result4.len());
        }

        None => {
            println!("Something went wrong!");
        }
    }
}


