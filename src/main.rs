use ncurses::*;

mod grid;
mod game;
mod display;

// Defer golang-like system implementation
// Reference: https://stackoverflow.com/questions/29963449/golang-like-defer-in-rust
struct ScopeCallback<F: FnMut()> {
  callback: F
}

impl<F: FnMut()> Drop for ScopeCallback<F> {
  fn drop(&mut self) {
      (self.callback)();
  }
}

macro_rules! defer {
    ($e: expr) => {
      let _scope_call = ScopeCallback { callback: || -> () { $e; } };
    };
}

fn main() {
  let dif = display::get_difficulty();
  display::start_display();
  let result = game::run_game(dif);
  endwin();
  display::handle_result(result);
}
