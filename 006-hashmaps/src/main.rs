use std::collections::HashMap;
use std::io;

fn main() {
    let mut text = String::new();

    println!("Word counter 1.0");
    println!("----------------");
    println!("Please enter a piece of text to have its words counted\n");

    io::stdin()
            .read_line(&mut text)
            .expect("Failed to read the line");
    
    let mut words: HashMap<String, i32> = HashMap::new();

    for word in text.split_whitespace(){
        let count = words.entry(String::from(word)).or_insert(0);
        *count += 1;
    }

    println!("\nResults:");

    for (word, count) in &words{
        if *count == 1 {
            println!("- '{}' appeared {} time", word, count);
        }
        else {
            println!("- '{}' appeared {} times", word, count);
        }
    }
}
