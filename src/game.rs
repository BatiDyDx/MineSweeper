use crate::display;
use rand::prelude::SliceRandom;

struct Action {
  x: u8,
  y: u8,
  mov: char // Action to be performed, i.e. uncheck, tag, quit, etc
}

enum GameState {
  Win,
  Loss,
  Continue
}

pub enum Difficulty {
  Easy,
  Medium,
  Hard
}

pub enum Content {
  Bomb,
  Free(u8)
}

pub enum State {
  Checked,
  Unchecked,
  Tagged
}

pub struct Cell {
  pub cont: Content,
  pub state: State
}

pub struct Grid {
  pub size: u8,
  pub mines: u8,
  pub unchecked_count: u8,
  pub cells: Vec<Vec<Cell>>
}

fn get_action(grid: &mut Grid) -> Action {
  Action { x: 0, y: 0, mov: 'q' }
}

fn get_game_state(grid: &mut Grid, action: Action) -> GameState {
  let (x, y, mov) = (action.x, action.y, action.mov);
  if mov == 'q' {
    todo!();
    return GameState::Loss;
  }
  
  let row = grid.cells
                                    .get_mut(y as usize)
                                    .unwrap();
  //grid.cells[action.x as usize][action.y as usize] = Cell;
  let cell = row
                                .get_mut(x as usize)
                                .unwrap();
  if mov == 'c' {
    match cell.state {
      State::Checked => (),
      _ => {
        if let Content::Bomb = cell.cont {
          return GameState::Loss;
        }
        cell.state = State::Unchecked;
      }
    }
  } else if mov == 't' {
    match cell.state {
      State::Checked => (),
      State::Unchecked => cell.state = State::Tagged,
      State::Tagged  => cell.state = State::Unchecked
    }
  }
  GameState::Continue
}

pub fn run_game(dif: Difficulty) -> bool {
  let mut grid = Grid::init_grid(dif);

  let result = loop {
    display::display_grid(&grid);
    let action = get_action(&mut grid);
    let state = get_game_state(&mut grid, action);
    match state {
      GameState::Loss => break false,
      GameState::Win  => break true,
      GameState::Continue => ()
    }
  };

  result
}

fn game_settings(difficulty: Difficulty) -> (u8, u8) {
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
  pub fn init_grid(difficulty: Difficulty) -> Grid {
    let (size, mines) = game_settings(difficulty);
    let mut positions: Vec<(u8,u8)> = vec![];
    for i in 1..size {
      for j in 1..size {
        positions.push((i,j));
      }
    }
    let mut rng = rand::thread_rng();
    
    let mine_positions: Vec<(u8,u8)> = positions
                            .choose_multiple(&mut rng, mines as usize)
                            .cloned()
                            .collect();
                            
    let mut cells: Vec<Vec<Cell>> = vec![];
    for i in 1..size {
      let mut row: Vec<Cell> = vec![];
      for j in 1..size {
        let cell;
        if mine_positions.contains(&(i,j)) {
          cell = Cell {cont: Content::Bomb, state: State::Unchecked};
        } else {
          cell = Cell {cont: Content::Free(0), state: State::Unchecked};
        }
        row.push(cell);
      }
      cells.push(row);
    }

    Grid {
      size, mines, unchecked_count: 0, cells
    }
  }
}
