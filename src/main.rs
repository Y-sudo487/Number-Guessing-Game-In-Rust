// Welcome to the Number Guessing Game!
// I'm thinking of a number between 1 and 100.
// You have 5 chances to guess the correct number.

// Please select the difficulty level:
// 1. Easy (10 chances)
// 2. Medium (5 chances)
// 3. Hard (3 chances)

// Enter your choice: 2

// Great! You have selected the Medium difficulty level.
// Let's start the game!

// Enter your guess: 50
// Incorrect! The number is less than 50.

// Enter your guess: 25
// Incorrect! The number is greater than 25.

// Enter your guess: 35
// Incorrect! The number is less than 35.

// Enter your guess: 30
// Congratulations! You guessed the correct number in 4 attempts.
extern crate rand;

use std::io;

fn main() {
  println!("Welcome to the Number Guessing Game!");
  println!("I'm thinking of a number between 1 and 100.");
  println!("Your chances depend on the difficulty of your choice.");
  println!("Please select the difficulty level:");
  println!("1. Easy (10 chances)");
  println!("2. Medium (5 chances)");
  println!("3. Hard (3 chances)");
  println!("Enter your choice:");

  let mut difficulty: String = String::new();

  let mut _chances: u8 = 0;

  let mut _attempts: u8 = 0;

  io::stdin()
    .read_line(&mut difficulty)
    .expect("Failed To Get Number.");

  let difficulty_u8: u8 = difficulty.trim().parse().expect("Failed to Get Number.");

  if difficulty_u8 == 1 {
    _chances = 10;
    println!("Great! you have selected the Easy difficulty.");
  } else if difficulty_u8 == 2 {
    _chances = 5;
    println!("Great! you have selected the Medium difficulty.");
  } else if difficulty_u8 == 3 {
    _chances = 3;
    println!("Great! you have selected the Hard difficulty.");
  }

  println!("Let's start the game!");
  for i in 1..=_chances {
    println!("{}", i);
  }
}
