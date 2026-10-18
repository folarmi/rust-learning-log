use std::io;

fn main() {
    println!("Welcome to the nth Fibonacci series Generator!");
    println!("Please input your value for n");

    let n: u64 = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            // Ok(num) => break num,
            Ok(num) => {
                if num <= 92 {
                    break num;
                }
                println!("n must be 92 or less, try again:");
            }
            Err(_) => println!("That's not a number, try again:"),
        }
    };


    let result = fibonacci(n);
    println!("The Fibonacci number for {n} is: {result}");
}

/// Returns the nth Fibonacci number: F(n) = F(n-1) + F(n-2)
fn fibonacci(n: u64) -> u64 {
    let mut prev = 0;
    let mut curr = 1;

    for _ in 0..n {
        (prev, curr) = (curr, prev + curr);
    }
    prev
}
