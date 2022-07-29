mod board;
mod position;

use board::Board;

#[deny(clippy::pedantic)]
fn main() {
    let board = Board::new_empty();
    println!("{board}");
}
