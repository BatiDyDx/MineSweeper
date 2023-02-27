use std::cmp::{min, max};
use crate::display;
use crate::grid::{Grid, State, Content};

pub enum GameState {
  Win,
  Loss,
  Continue,
  Quit
}

pub struct Position {
  pub x: usize,
  pub y: usize
}

pub enum Movement {
  Up, Down, Left, Right,  // Movements across the board
  Cord(Position),         // Movement representing a new position
  Check,                  // Check a cell
  Tag,                    // Tag/Untag a cell
  Quit                    // Quit game
  // Restart maybe?
}

pub enum Difficulty {
  Easy,
  Medium,
  Hard
}

pub fn run_game(dif: Difficulty) -> GameState {
  let (size, mine_count) = game_settings(dif);
  let mut grid = Grid::new(size, mine_count);
  grid.set();

  let mut pos: Position = Position {x: 0, y: 0};
  let mut state = GameState::Continue;

  while let GameState::Continue = state {
    display::update_display(&grid, (pos.x, pos.y));
    let mov = display::get_input();
    match mov {
      // Its done this way to avoid overflow
      Movement::Up => pos.y = max(1, pos.y) - 1,
      Movement::Left => pos.x = max(1, pos.x) - 1,
      Movement::Down => pos.y = min(grid.size - 1, pos.y + 1),
      Movement::Right => pos.x = min(grid.size - 1, pos.x + 1),
      Movement::Cord(new_pos) => pos = new_pos,
      Movement::Quit => state = GameState::Quit,
      Movement::Tag => grid.tag_cell(&pos),
      Movement::Check => state = grid.uncheck_cell(&pos)
    }
  }

  grid.display_solved_grid();
  // TODO: interfaz previa a quitar el juego
  state
}

fn game_settings(difficulty: Difficulty) -> (usize, usize) {
  return match difficulty {
    Difficulty::Easy => (10, 15),
    Difficulty::Medium => (15, 30),
    Difficulty::Hard => (30, 100)
  }
}

impl Difficulty {
  pub fn from_int(n: u8) -> Self {
    return match n {
      1 => Difficulty::Easy,
      2 => Difficulty::Medium,
      _ => Difficulty::Hard
    }
  }
}

impl Grid {
  fn tag_cell(self: &mut Self, pos: &Position) {
    let cell = self.cells
                  .get_mut(pos.y)
                  .unwrap()
                  .get_mut(pos.x)
                  .unwrap();
    match cell.state {
      State::Checked => (),
      State::Tagged => cell.state = State::Unchecked,
      State::Unchecked => cell.state = State::Tagged
    }
  }

  fn uncheck_cell(self: &mut Self, pos: &Position) -> GameState {
    let cell = self.cells
                          .get_mut(pos.y)
                          .unwrap()
                          .get_mut(pos.x)
                          .unwrap();
    match cell.state {
      State::Unchecked => {
        cell.state = State::Checked;
        if let Content::Bomb = cell.cont {
          return GameState::Loss;
        }
        self.unchecked_count += 1;
      },
      _ => ()
    }
    if self.unchecked_count + self.mine_count == self.size * self.size {
      return GameState::Win;
    }
    GameState::Continue
  }
  
  pub fn update_grid() {
    ()
  }
  
}
