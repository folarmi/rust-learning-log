// Using a hash map and vectors, create a text interface to allow a user
// to add employee names to a department in a company; for example,
// “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the
// user retrieve a list of all people in a department or all people
// in the company by department, sorted alphabetically

use ::std::collections::HashMap;
use std::io;

fn main() {
    employee_directory();
}

fn employee_directory() {
    println!("Build your Employee directory!!!");
    let mut store: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        // println!("Add a name to the directory");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        // Collect into a Vector of string slices (&str)
        let words: Vec<&str> = user_input.split_whitespace().collect();
        if words.len() == 0 {
            break;
        };

        if words[0].eq_ignore_ascii_case("Add") {
            let dept = store.entry(String::from(words[3])).or_default();
            dept.push(String::from(words[1]));
        } else if words.len() == 2 {
            let dept = String::from(words[1]);

            match store.get_mut(&dept) {
                Some(value) => {
                    value.sort_unstable();
                    println!("{value:?}")
                }
                None => (),
            }
        } else if words[0] == "list" {
            for (key, value) in &mut store {
                value.sort_unstable();
                println!("{key}: {value:?}")
            }
        }

        if user_input.trim() == "quit" {
            break;
        }
    }
}
