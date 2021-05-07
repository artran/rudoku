#![warn(clippy::all, clippy::pedantic)]

mod board;
mod cell_group;

use crate::board::Board;


fn main() {
    let brd = Board::from_string();

    brd.solve();
}
