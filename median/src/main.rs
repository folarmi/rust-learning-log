// Given a list of integers, use a vector and return the median (when sorted,
// the value in the middle position) and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.
use std::collections::HashMap;

fn main() {
    let test = vec![1, 2, 2, 2, 5, 6];

    let value = median_and_mode(test);
    println!("The result is {value:?}")
}

fn median_and_mode(mut input: Vec<i8>) -> (f64, i8) {
    input.sort();
    let vec_len = input.len();
    let midpoint = vec_len / 2;
    let median: f64 = if vec_len % 2 == 0 {
        let a = input[midpoint - 1] as f64;
        let b = input[midpoint] as f64;
        (a + b) / 2.0
    } else {
        input[midpoint] as f64
    };

    let mut counts: HashMap<i8, _> = HashMap::new();
    for i in &input {
        let count = counts.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut mode:i8 = 0;
    let mut best_count = 0;
    for (key, value) in &counts {
        if *value > best_count {
            best_count = *value;
            mode = *key;
        }

    }

    (median, mode)
}


