use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);
    println!("Welcome to Guess the Number, the official game where you have to guess the number to win.\nIt can be any number from 1 to 100.\nLet's get started!\n");
    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                print!("That one isn't valid. ");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small! Try another one"),
            Ordering::Greater => println!("Too big! Try another one"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
