fn main() {
    println!("Hello, world!");

    // another_function(900);
    // print_labeled_measurement(5, 'h');
    add_one(590);
}

// fn another_function (x:i32) {
//     println!("The value of x is: {x}");
// }


// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

fn add_one (x:i32) -> i32 {
    let result = x + 1;
    println!("The result of add_one is  : {result}");
    result;
}


