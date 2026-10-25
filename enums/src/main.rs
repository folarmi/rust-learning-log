enum IpAddrKind {
    V4,
    V6,
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String
// }

 #[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move {x;i32, y :i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}



fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // }

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address:String::from("::1")
    // }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home:{home:?}, loopback:{loopback:?}")
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}

if let Some(max) = config_max {
    println!("The maximum is configured to be {max}"),
}