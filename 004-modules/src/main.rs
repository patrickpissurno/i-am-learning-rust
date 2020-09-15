mod utils;
use utils::greet;

fn main() {
    greet();

    let x = 20;
    println!("X was {}", x);

    let x = utils::math::invert_signal(x);
    println!("X is now {}", x);
}