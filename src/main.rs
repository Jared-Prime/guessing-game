use std::old_io;
use std::rand;

fn main() {
    println!("Guess a number between 1 and 100");

    // generate random unsigned 32 bit integer, take mod 100, add one
    let secret = (rand::random::<u32>() % 100) + 1;

    println!("Please input your guess");

    let input = old_io::stdin()
                    .read_line()
                    .ok()
                    .expect("Failed to read line");

    println!("You guessed: {}", input);
}
