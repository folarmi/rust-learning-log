// Convert strings to Pig Latin. The first consonant of each word is moved to the
// end of the word and ay is added, so first becomes irst-fay. Words that
// start with a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!

fn main() {
    let test = String::from("first apple");

    let value = pig_latin(test);
    println!("{value}")
}

fn pig_latin(mut input: String) -> String {
    input.make_ascii_lowercase();
    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'y', 'z',
    ];

    let mut collected_words: Vec<String> = Vec::new();

    for word in input.split_whitespace() {
        let letter = word.chars().next();
        match letter {
            Some(first) => {
                if consonants.contains(&first) {
                    let rest = &word[first.len_utf8()..];
                    collected_words.push(format!("{rest}-{first}ay"));
                } else {
                    collected_words.push(format!("{word}-hay"));
                }
            }
            None => (),
        }
    }

    collected_words.join(" ")
}
