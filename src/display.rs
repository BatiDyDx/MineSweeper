use crate::game::{GameState, Difficulty, Movement};
use crate::grid::{Grid, Content, State};
use std::io::{self,Write};
use ncurses::*;

const FILL_FMT: usize = 4;
const HIGHLIGHT_PAIR: i16 = 1;
const REGULAR_PAIR: i16 = 2;

macro_rules! grid_symbol {
  ($symbol:expr) => {
    format!("{:^fill$}", $symbol, fill = FILL_FMT)
  }
}

pub fn get_difficulty() -> Difficulty {
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

pub fn handle_result(result: GameState) {
  match result {
    GameState::Win => println!("YOU WIN!!!"),
    GameState::Loss => println!("It's a defeat"),
    GameState::Quit => println!("Exiting..."),
    GameState::Continue => panic!("This is impossible")
  }
}

// Version with coordinate parsing implemented
// pub fn get_input() -> Movement {
//   let mut buffer = String::new();
//   loop {
//     getstr(&mut buffer);
//     // let c = getch() as u8 as char;
//     let inputs = buffer
//                                         .split_ascii_whitespace()
//                                         .collect::<Vec<&str>>();
//     let len = inputs.len();
//     if len == 2 {
//       let x = inputs.get(0).unwrap().parse::<usize>();
//       let y = inputs.get(1).unwrap().parse::<usize>();
//       if !x.is_err() && !y.is_err() {
//         break Movement::Cord(Position {x: x.unwrap(), y: y.unwrap()});
//       }
//     } else if len == 1 {
//       let c = inputs.get(0).unwrap().parse::<char>();
//       if c.is_err() { continue; }
//       break match c.unwrap() {
//         'q' => Movement::Quit,
//         'c' => Movement::Check,
//         't' => Movement::Tag,
//         'a' => Movement::Left,
//         'd' => Movement::Right,
//         'w' => Movement::Up,
//         's' => Movement::Down,
//         _ => continue
//       }
//     }
//   }
// }


pub fn get_input() -> Movement {
  loop {
    let c = getch() as u8 as char;
    break match c {
      'q' => Movement::Quit,
      'c' => Movement::Check,
      't' => Movement::Tag,
      'a' => Movement::Left,
      'd' => Movement::Right,
      'w' => Movement::Up,
      's' => Movement::Down,
      _ => continue
    }
  }
}


pub fn start_display() {
  initscr();
  noecho();
  start_color();
  init_pair(2, COLOR_BLACK, COLOR_BLACK);
  init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
  init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

pub fn update_display(grid: &Grid, pos: (usize, usize)) {
  attron(COLOR_PAIR(REGULAR_PAIR));
  mv(1,1);
  addstr("Mine Sweeper Game\n");
  for (j, row) in grid.cells.iter().enumerate() {
    //mv((i + 1) as i32, 1);
    addstr(&format!("{:>2} | ", j));
    for (i, cell) in row.iter().enumerate() {
      if (i, j) == pos {
        attroff(COLOR_PAIR(REGULAR_PAIR));
        attron(COLOR_PAIR(HIGHLIGHT_PAIR));
      }
      match &cell.state {
        State::Checked => {
          match cell.cont {
            // This function should receive a grid with no bombs discovered
            Content::Bomb => panic!("This is impossible"),
            Content::Free(n) => { addstr(&grid_symbol!(n)); () }
          }
        }
        State::Tagged => { addstr(&grid_symbol!("X")); () },
        State::Unchecked => { addstr(&grid_symbol!("-")); () }
      }
      if (i, j) == pos {
        attroff(COLOR_PAIR(HIGHLIGHT_PAIR));
        attron(COLOR_PAIR(REGULAR_PAIR));
      }
    }
    addstr("\n");
  }
  addstr("     ");
  for i in 0..grid.size {
    addstr(&grid_symbol!(i));
  }
  addstr("\n");
  refresh();
}


impl Grid {
  pub fn display_solved_grid(self: &Self) {
    ()
  }
}
