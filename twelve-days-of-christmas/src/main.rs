fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for index in 0..12 {
        println!();
        let day = days[index];
        println!("On the {day} day of Christmas");
        println!("My true love gave to me");

        for g in (0..=index).rev() {
            let gift = gifts[g];

            if g == 0 && index > 0 {
                println!("And a partridge in a pear tree");
            } else {
                println!("{gift}");
            }
        }
    }
}
