mod sudoku;

use sudoku::{Board, backtrack};


fn main() {
    let board = Board::parse_puzzle(std::io::stdin());
    if let Some(solved) = backtrack(board, 0, 0) {
        println!("{}", solved.to_string());
    } else {
        println!("Could not solve.");
    }
}

