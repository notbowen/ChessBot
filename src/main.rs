// Main file to run the engine
// We'll improve the comments later

use crate::board::Board;
use crate::misc::display;

mod board;
mod defs;
mod state;
mod bits;
mod misc;

fn main() {
    let mut board = Board::new();
    match board.fen_setup(None) {
        Ok(_) => println!("Generated default FEN moves!"),
        Err(_) => println!("Something went wrong!"),
    }

    display::position(&board, None);
}
