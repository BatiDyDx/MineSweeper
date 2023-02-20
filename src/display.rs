use crate::game::Grid;
use crate::game::{Content,State};

pub fn display_grid(grid: &Grid) {
  for (i, row) in grid.cells.iter().enumerate() {
    print!("{} | ", i + 1);
    for cell in row.iter() {
      match &cell.state {
        State::Checked => {
          match cell.cont {
            Content::Bomb => print!(" * "),
            Content::Free(n) => print!(" {n} ")
          }
        }
        State::Tagged => print!(" X "),
        State::Unchecked => print!(" - ")
      }
    }
    println!("");
  }
  print!("    ");
  for i in 1..grid.size {
    print!(" {i} ");
  }
  println!("");
}
