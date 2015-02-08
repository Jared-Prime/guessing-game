use std::old_io;
use std::rand;

fn main() {
    println!("Guess a number between 1 and 100");

    println!("Please input your guess");

    let input = old_io::stdin()
                    .read_line()
                    .ok()
                    .expect("Failed to read line");

    println!("You guessed: {}", input);
}
