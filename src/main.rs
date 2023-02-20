use std::io::{self, Write};

use game::Difficulty;
mod game;
mod display;

fn get_difficulty() -> Difficulty {
  print!("Ingrese la dificultad [1,2,3]: ");
  let _ = io::stdout().flush();
  let stdin = io::stdin();
  let mut input_dif = String::new();
  stdin.read_line(&mut input_dif).expect("Could not read stdio");
  let input_dif = input_dif
                      .trim()
                      .parse::<u8>()
                      .expect("Input not valid");
  Difficulty::from_int(input_dif)
}

fn main() {
  let result = game::run_game(get_difficulty());
  if result {
    println!("YOU WIN!!");
  } else {
    println!("It's a defeat");
  }
}

