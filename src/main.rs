extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("The secrete number is: {}", secret_number);

  println!("Please input your guess.");

  // The ::new() syntax uses :: because this is an ‘associated function’ of a
  // particular type. That is to say, it’s associated with String itself, rather
  // than a particular instance of a String. Some languages call this a ‘static
  // method’.
  let mut guess = String::new();

  io::stdin().read_line(&mut guess)
    .ok()
    .expect("Failed to read line");

  let guess: u32 = guess.trim().parse()
    .ok()
    .expect("Please type a number!");

  println!("You guessed: {}", guess);

  match guess.cmp(&secret_number) {
    Ordering::Less    => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal   => println!("You win!"),
  }
}
