use rand::RngExt;
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

  let mut chances: u32 = 0;

  let mut attempts: u32 = 0;

  io::stdin()
    .read_line(&mut difficulty)
    .expect("Failed To Get Number.");

  let difficulty_u32: u32 = difficulty.trim().parse().expect("Failed to Get Number.");

  if difficulty_u32 == 1 {
    chances = 10;
    println!("Great! You have selected the Easy difficulty level.");
  } else if difficulty_u32 == 2 {
    chances = 5;
    println!("Great! You have selected the Medium difficulty level.");
  } else if difficulty_u32 == 3 {
    chances = 3;
    println!("Great! You have selected the Hard difficulty level.");
  }

  println!("Let's start the game!");

  let rand_num: u32 = rand::rng().random_range(1..=100);

  for _i in 1..=chances {
    println!("Enter your guess:");

    let mut guess_str: String = String::new();
    io::stdin()
      .read_line(&mut guess_str)
      .expect("Failed To Get Guess.");

    let guess: u32 = guess_str.trim().parse().expect("Failed to get Number.");

    attempts += 1;

    if guess == rand_num {
      println!(
        "Congratulations! You guessed the correct number in {} attempts.",
        attempts
      );
      break;
    } else if guess > rand_num {
      println!("Incorrect! The number is less than {}.", guess);
    } else {
      println!("Incorrect! The number is greater than {}.", guess);
    }
  }
}
