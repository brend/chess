use crate::chess::Board;

mod chess;

fn main() {
    let board = Board::new();
    board.print();
}
