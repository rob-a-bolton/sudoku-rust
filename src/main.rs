mod sudoku;

#[macro_use]
extern crate clap;
use sudoku::{Board, backtrack};
use clap::App;
use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input_path = matches.value_of("input").unwrap();

    if let Ok(file) = File::open(input_path) {
        let mut lines = BufReader::new(file).lines();
        while let Ok((board, rest_lines)) = Board::parse_puzzle(lines) {
            lines = rest_lines;
            if let Some(solved) = backtrack(board, 0, 0) {
                println!("{}", solved.to_string());
            } else {
                println!("Could not solve.");
            }
        }
    } else {
        println!("Could not read input file {}", input_path);
    }
}
