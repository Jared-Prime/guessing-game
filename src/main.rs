use std::old_io;
use std::rand;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number between 1 and 100");

    // generate random unsigned 32 bit integer, take mod 100, add one
    let secret = (rand::random::<u32>() % 100) + 1;

    println!("Please input your guess");

    let input = old_io::stdin()
                    .read_line()
                    .ok()
                    .expect("Failed to read line");

    // change our &str input to an unsigned 32 bit integer
    let guess: Option<u32> = input.trim().parse().ok();

    // we need to unwrap the parsed input, verify it as an integer
    let guess_num = match guess {
        Some(guess) => guess,
        None => {
          println!("please input a number!");
          return;
        }
    };

    match cmp(secret, guess_num) {
      Ordering::Less => println!("too small!"),
      Ordering::Greater => println!("too big!"),
      Ordering::Equal => println!("you win!"),
    }
}

fn cmp(a: u32, b: u32) -> Ordering {
  if a < b { Ordering::Less }
  else if a > b { Ordering::Greater }
  else { Ordering::Equal }
}
