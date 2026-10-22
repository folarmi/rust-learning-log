fn main() {
    // let mut s = String::from("hello world");
    let s = "hxello world";

    let word = first_word(&s);

    s.clear();
    println!("{word}")

    // let s = String::from("hi");

    // add_bang(&mut s);

    let mut s = String::from("hello world");
    let word = first_word(&s);
    
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn add_bang(s: &mut String) {
    s.push("!");
} 