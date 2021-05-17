#![warn(clippy::all, clippy::pedantic)]

mod board;
mod cell_group;

use crate::board::Board;


fn main() {
    let mut board = Board::from_string(&"600003000\
            090867300\
            037250600\
            004130200\
            309000105\
            008029400\
            005041720\
            003796080\
            000500001");

    board.solve();

    println!("{}", board.print());
}
