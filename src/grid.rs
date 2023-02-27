use rand::Rng;
use crate::game::Position;

#[derive(Debug, Clone)]
pub enum Content {
  Bomb,
  Free(u8) // Number of neighbour bombs
}

#[derive(Debug, Clone)]
pub enum State {
  Checked,
  Unchecked,
  Tagged
}

#[derive(Debug, Clone)]
pub struct Cell {
  pub cont: Content,
  pub state: State
}

pub struct Grid {
  pub size: usize,
  pub mine_count: usize,
  pub unchecked_count: usize,
  pub cells: Vec<Vec<Cell>>
}


fn neighbour_is_bomb(cells: &Vec<Vec<Cell>>, pos: (usize, usize)) -> bool {
  let (x, y) = pos;
  let cell = cells.get(y).unwrap().get(x).unwrap();
  match cell.cont {
    Content::Bomb => true,
    Content::Free(_) => false
  }
}

impl Grid {
  pub fn new(size: usize, mine_count: usize) -> Grid {
    let mut cells: Vec<Vec<Cell>> = vec![];
    
    for _ in 0..size {
      let mut row: Vec<Cell> = vec![];
      for _ in 0..size {
        let cell;
        cell = Cell {cont: Content::Free(0), state: State::Unchecked};
        row.push(cell);
      }
      cells.push(row);
    }
    
    Grid { size, mine_count, unchecked_count: 0, cells }
  }

  pub fn set(self: &mut Self) {
    let mut rng = rand::thread_rng(); 
    let mut mines_located = 0;
    while mines_located < self.mine_count {
      let (i, j) = rng.gen::<(usize,usize)>();
      let (i, j) = (i % self.size, j % self.size);
      let row = self.cells.get_mut(j).unwrap();
      let cell = row.get_mut(i).unwrap();
      match &cell.cont {
        Content::Bomb => (),
        Content::Free(_) => {
          cell.cont = Content::Bomb;
          mines_located += 1;
        }
      }
    }

    let copy = self.cells.clone();

    for (j, row) in self.cells.iter_mut().enumerate() {
      for (i, cell) in row.iter_mut().enumerate() {
        if let Content::Bomb = cell.cont {
          continue;
        }
        
        let mut neigh: Vec<bool> = vec![];
        // Add all cells neighbour to the vector
        let left = i == 0;
        let right = i == self.size - 1;
        let bottom = j == self.size - 1;
        let top = j == 0;
        if !left {
          if !bottom {
            neigh.push(neighbour_is_bomb(&copy, (i - 1, j + 1)));
          }
          if !top {
            neigh.push(neighbour_is_bomb(&copy, (i - 1, j - 1)));
          }
          neigh.push(neighbour_is_bomb(&copy, (i - 1, j)));
        } if !right {
          if !bottom {
            neigh.push(neighbour_is_bomb(&copy, (i + 1, j + 1)));
          }
          if !top {
            neigh.push(neighbour_is_bomb(&copy, (i + 1, j - 1)));
          }
          neigh.push(neighbour_is_bomb(&copy, (i + 1, j)));
        }
        if !top {
          neigh.push(neighbour_is_bomb(&copy, (i, j - 1)));
        }
        if !bottom {
          neigh.push(neighbour_is_bomb(&copy, (i, j + 1)));
        }

        let neighbour_bombs = neigh.into_iter()
            .filter(|b| *b)
            .collect::<Vec<bool>>()
            .len();

        cell.cont = Content::Free(neighbour_bombs as u8);
      }
    }
  }

}
