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

use std::io;
use rand::RngExt;

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

  let mut chances: u32 = 0;

  let mut attempts: u32 = 0;

  io::stdin()
    .read_line(&mut difficulty)
    .expect("Failed To Get Number.");

  let difficulty_u32: u32 = difficulty.trim().parse().expect("Failed to Get Number.");

  let mut guess_str: String = String::new();

  if difficulty_u32 == 1 {
    chances = 10;
    println!("Great! you have selected the Easy difficulty.");
  } else if difficulty_u32 == 2 {
    chances = 5;
    println!("Great! you have selected the Medium difficulty.");
  } else if difficulty_u32 == 3 {
    chances = 3;
    println!("Great! you have selected the Hard difficulty.");
  }

  println!("Let's start the game!");
  println!("Enter Your Guess: ");
  io::stdin()
    .read_line(&mut guess_str)
    .expect("Failed To Get Guess.");

  let mut guess: u32 = guess_str
    .trim()
    .parse()
    .expect("Failed to Get Number Guess.");

  let rand_num: u32 = rand::rng().random_range(1..=100);



  for _i in 1..=chances {

    if guess == rand_num {
      
      attempts += 1;
      
      println!(
        "Congratulations! You guessed the correct number in {} attempts.",
        attempts
      );
      
      break;
    } 

    else if guess > rand_num {
      
      attempts += 1;
      
      println!("Incorrect! The number is less than {}.", guess);
      
      guess_str.clear();
      
      io::stdin()
        .read_line(&mut guess_str)
        .expect("Failed To Get Guess.");
      
      guess= guess_str.trim().parse().expect("Failed to get Number.");
    } 

    else if guess < rand_num {
      
      attempts += 1;
      
      println!("Incorrect! The number is greater than {}.", guess);
      
      guess_str.clear();
      
      io::stdin()
        .read_line(&mut guess_str)
        .expect("Failed To Get Guess.");

      guess= guess_str.trim().parse().expect("Failed to get Number.");
    } 
    
    else {
      break;
    }

  }
  
}
