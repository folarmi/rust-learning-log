use std::io;

fn main() {
    println!("Enter a temperature in Fahrenheit:");

    let temperature: f64 = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("That's not a number, try again:"),
        }
    };

    let celsius = (temperature - 32.0) * 5.0 / 9.0;

    println!("Temperature in Celsius is: {celsius:.1}");
}
