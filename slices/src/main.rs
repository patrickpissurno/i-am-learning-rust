fn main() {
    let mut s = String::from("hello world");

    println!("s: '{}'", s);

    let word = first_word(&s);

    println!("word: '{}'", word);
    s.clear();

    println!("s: '{}'", s);
    // println!("word: {}", word); //would result in a compiler error, because word is a immutable reference and s has changed in the meantime
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
