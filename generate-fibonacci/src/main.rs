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

/// Returns the nth Fibonacci number, counting from F(0) = 0.
///
/// F(0) = 0, F(1) = 1, and every number after is the sum of the two before it:
/// F(n) = F(n-1) + F(n-2)
///
/// How it works: we only ever keep the last two numbers, `prev` and `curr`,
/// and slide them forward one step per loop. After n steps, `prev` holds F(n).
///
/// | step  | prev | curr |
/// |-------|------|------|
/// | start | 0    | 1    |
/// | 1     | 1    | 1    |
/// | 2     | 1    | 2    |
/// | 3     | 2    | 3    |
///
/// Limit: n must be 92 or less. Each step also calculates the NEXT number,
/// so for n = 93 the loop tries to compute F(94), which is too big for u64.
fn fibonacci(n: u64) -> u64 {
    let mut prev = 0;
    let mut curr = 1;

    for _ in 0..n {
        // Right side is calculated FIRST, using the old values:
        //   new prev = old curr
        //   new curr = old prev + old curr  (the sum, i.e. the "next" number)
        // Then both are assigned at once, so nothing is overwritten too early.
        // Same as JS destructuring: [prev, curr] = [curr, prev + curr];
        (prev, curr) = (curr, prev + curr);
    }

    prev
}
