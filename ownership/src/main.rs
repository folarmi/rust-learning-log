fn main() {
    // let mut s = String::from("hello");
    // s.push_str(",world!");
    // println!("{s}");

    // let mut s2 = "hello";
    // s2 = "This is a longer string";
    // println!("{s2}");

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {s1}, s2 = {s2}");

    // let s = String::from("hello");

    // takes_ownership(s);

    // let x = 5;

    // makes_copy(x);

    // let value = String::from("hello");

    // let (s2, len) = calculate_length(value);

    // println!("The length of '{s2}' is {len}.");

    // let len = calculate_length(&value);
    // println!("The length of '{value}' is {len}")

    // let mut s = String::from("hello");
    // change(&mut s);

    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{r1} and {r2}");
    // // Variables r1 and r2 will not be used after this point.

    // let r3 = &mut s; // no problem
    // println!("{r3}");

    let reference_to_nothing = dangle();

}

// fn takes_ownership(some_string: String) {
//     println!("{some_string}")
// }

// fn makes_copy(some_integer: i32) {
//     println!("{some_integer}")
// }

// fn calculate_length(s:String) -> (String, usize) {
//     let length = s.len();

//     (s,length)
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world")
// }

fn dangle () -> &String {
    let s = String::from("hello");

    &s
}