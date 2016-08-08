use std::char::from_digit;
use std::collections::HashSet;
use std::io::{Read, BufReader, BufRead};

#[derive(Debug, Copy, Clone, PartialEq)]
/// Represents a single cell on the board.
/// Can be either Open (unsoved), Fixed (one
/// of the cells set as part of the puzzle)
/// or Maybe (a value currently being tried)
enum Cell {
    Fixed(u8),
    Maybe(u8),
    Open,
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match *self {
            Cell::Fixed(num) => {
                num.to_string()
            },
            Cell::Maybe(num) => {
                num.to_string()
            },
            Cell::Open => ' '.to_string(),
        }
    }
}

/// Used to store the puzzle's state
struct Board {
    grid: [[Cell; 9]; 9]
}

impl Board {
    /// Instantiates a new board.
    fn new(values: Vec<(usize, usize, u8)>) -> Self {
        let mut grid = [[Cell::Open; 9],
                        [Cell::Open; 9],
                        [Cell::Open; 9],
                        [Cell::Open; 9],
                        [Cell::Open; 9],
                        [Cell::Open; 9],
                        [Cell::Open; 9],
                        [Cell::Open; 9],
                        [Cell::Open; 9]];
        
        for (x, y, val) in values.into_iter() {
            grid[x][y] = Cell::Fixed(val);
        }
        
        Board {
            grid: grid
        }
    }

    fn patch(&self, x: usize, y: usize, cell: Cell) -> Self {
        let mut grid = self.grid.clone();

        grid[y][x] = cell;
        
        Board {
            grid: grid,
        }
    }

    /// Gets all possible numbers for a cell, taking into account
    /// the row, column, and box the cell resides in.
    fn get_possible(&self, x: usize, y: usize) -> HashSet<u8> {
        let mut nums: HashSet<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();

        // Search across a row
        for x in 0..9 {
            match self.grid[y][x] {
                Cell::Fixed(num) | Cell::Maybe(num) => {
                    nums.remove(&num);
                },
                Cell::Open => continue,
            }
        }

        // Search across a column
        for y in 0..9 {
            match self.grid[y][x] {
                Cell::Fixed(num) | Cell::Maybe(num) => {
                    nums.remove(&num);
                },
                Cell::Open => continue,
            }
        }

        // Search across a box
        let grid_x = x / 3;
        let grid_y = y / 3;
        for y in 0 .. 3 {
            for x in 0 .. 3 {
                match self.grid[y + (grid_y * 3)][x + (grid_x * 3)] {
                    Cell::Fixed(num) | Cell::Maybe(num) => {
                        nums.remove(&num);
                    },
                    Cell::Open => continue,
                }      
            }
        }

        nums
    }

    /// Parses a puzzle from an input source
    /// This is expected to be ascii-encoded,
    /// representing empty cells with spaces
    fn parse_puzzle<T: Read>(input: T) -> Self {
        let input = BufReader::new(input);
        let mut grid = [[Cell::Open; 9]; 9];
        let mut y = 0;

        for line in input.lines().take(9) {
            let mut x = 0;
            for chr in line.unwrap().chars().into_iter() {
                grid[y][x] = match chr {
                    chr @ '1' ... '9' => {
                        Cell::Fixed(chr as u32 as u8 - 48)
                    },
                    _ => {
                        Cell::Open
                    },
                };
                
                x = x + 1;
            }
            y = y + 1;
        }

        Board {
            grid: grid,
        }
    }
}


impl ToString for Board {
    fn to_string(&self) -> String {
        let mut board_str = String::new();

        for row in self.grid.iter() {
            for cell in row.iter() {
                board_str.push(
                    match *cell {
                        Cell::Fixed(num) => {
                            from_digit(num as u32, 10).unwrap()
                        },
                        Cell::Maybe(num) => {
                            from_digit(num as u32, 10).unwrap()
                        },
                        Cell::Open => ' ',
                    }
                );
            }
            board_str.push('\n');
        }

        board_str
    }
}

fn main() {
    let board = Board::parse_puzzle(std::io::stdin());
    if let Some(solved) = backtrack(board, 0, 0) {
        println!("{}", solved.to_string());
    } else {
        println!("Could not solve.");
    }
}

/// Attempts to solve a sudoku board with backtrack
/// brute-forcing.
fn backtrack(board: Board, x: usize, y: usize) -> Option<Board> {
    let (next_x, next_y) = next_coords(x, y);
    if y == 9 {
        return Some(board);
    }
    match board.grid[y][x] {
        Cell::Fixed(_) | Cell::Maybe(_) => {
            backtrack(board, next_x, next_y)
        },
        Cell::Open => {
            let nums = board.get_possible(x, y);
            
            for num in nums.iter() {
                if let Some(solved) = backtrack(board.patch(x, y, Cell::Maybe(*num)), next_x, next_y) {
                    return Some(solved);
                }
            }

            None
        },
    }
}

/// Increments a pair of coordinates.
/// Adds 1 to the x, setting it back to 0 and incrementing
/// y after x reaches 8
fn next_coords(x: usize, y: usize) -> (usize, usize) {
    let mut y = y;
    let x = match x {
        8 => {
            y = y + 1;
            0
        },
        _ => x + 1
    };
    
    (x, y)
}


#[cfg(test)]
#[test]
fn test_patch() {
    let board = Board::new(
        vec!((0, 0, 1))
    ).patch(1, 0, Cell::Fixed(2));

    assert_eq!(Cell::Fixed(1), board.grid[0][0]);
    assert_eq!(Cell::Fixed(2), board.grid[0][1]);
}

#[test]
fn test_parse_puzzle() {
    let puzzle_str =
"  39   51
546 183  
     742 
  9 5  3 
2  6 3  4
 8  7 2  
 973     
  182 947
85   46  
";
    
    let puzzle = Board::parse_puzzle(puzzle_str.as_bytes());
    assert_eq!(puzzle_str, puzzle.to_string());
}

#[test]
fn test_coords() {
    assert_eq!((1, 0), next_coords(0, 0));
    assert_eq!((0, 1), next_coords(8, 0));
}

#[test]
fn test_get_possible() {
    let board = Board::parse_puzzle(
" 23456789
2        
3        
4        
5        
6        
7        
8        
9        ".as_bytes());

    let expected: HashSet<u8> = vec![1].into_iter().collect();
    assert_eq!(expected, board.get_possible(0, 0));
}
