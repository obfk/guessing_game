use std::io;

fn main() {
  println!("Guess the number!");

  // The ::new() syntax uses :: because this is an ‘associated function’ of a
  // particular type. That is to say, it’s associated with String itself, rather
  // than a particular instance of a String. Some languages call this a ‘static
  // method’.
  let mut guess = String::new();

  io::stdin().read_line(&mut guess)
    .ok()
    .expect("Failed to read line");

  print!("You guessed: {}", guess);
}
