use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let _v: Vec<i32> = Vec::new();

    let _v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let _does_not_exist = &v4[100];
    // let _does_not_exist = v4.get(100);

    let v5 = vec![1, 2, 3, 4, 5];
    let first = &v5[0];

    // v5.push(6);

    println!("The first element is: {first}");

    for i in &v5 {
        println!("{i}");
    }

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    let mut s = String::from("lo");
    let s2 = 'l';
    s.push(s2);
    println!("s2 is {s2}");

    let s1 = String::from("tic");
    let s3 = String::from("tac");
    let s4 = String::from("toe");

    let solid = format!("{s1}-{s3}-{s4}");
    println!("{solid}");

    for b in "Зд".bytes() {
        println!("{b}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"),50);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }

    println!("{map:?}");
}
