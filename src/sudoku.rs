use std::char::from_digit;
use std::io::{BufRead, Lines};

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
pub struct Board {
    grid: [[Cell; 9]; 9]
}

impl Board {
    /// Creates a clone of a board, replacing a single cell with a
    /// new one.
    fn patch(&self, x: usize, y: usize, cell: Cell) -> Self {
        let mut grid = self.grid.clone();

        grid[y][x] = cell;
        
        Board {
            grid: grid,
        }
    }

    /// Gets all possible numbers for a cell, taking into account
    /// the row, column, and box the cell resides in.
    fn get_possible(&self, x: usize, y: usize) -> Vec<u8> {
        let mut nums: Vec<bool> = vec![true; 9];
        
        // Search across a row
        for x in 0..9 {
            match self.grid[y][x] {
                Cell::Fixed(num) | Cell::Maybe(num) => {
                    nums[(num - 1) as usize] = false;
                },
                Cell::Open => continue,
            }
        }

        // Search across a column
        for y in 0..9 {
            match self.grid[y][x] {
                Cell::Fixed(num) | Cell::Maybe(num) => {
                    nums[(num - 1) as usize] = false;
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
                        nums[(num - 1) as usize] = false;
                    },
                    Cell::Open => continue,
                }      
            }
        }

        let mut ret_nums: Vec<u8> = Vec::with_capacity(9);
        let mut n = 1;
        for num in nums.into_iter() {
            if num {
                ret_nums.push(n);
            }
            
            n += 1;
        }

        ret_nums
    }

    /// Parses a puzzle from an input source
    /// This is expected to be ascii-encoded,
    /// representing empty cells with spaces
    pub fn parse_puzzle<B: BufRead>(mut lines: Lines<B>) -> Result<(Self, Lines<B>), &'static str> {
        let mut grid = [[Cell::Open; 9]; 9];

        for y in 0..9 {
            let mut x = 0;
            if let Some(Ok(line)) = lines.next() {
                for chr in line.chars().into_iter().take(9) {
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
            } else {
                return Err("Reached end of input.");
            }
        }

        // Read a blank line
        lines.next();

        let board = Board { grid: grid };
        Ok((board, lines))
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


/// Attempts to solve a sudoku board with backtrack
/// brute-forcing.
pub fn backtrack(board: Board, x: usize, y: usize) -> Option<Board> {
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
use std::io::BufReader;

#[test]
fn test_patch() {
    let lines = BufReader::new("1\n\n\n\n\n\n\n\n\n".as_bytes()).lines();
    let board = Board::parse_puzzle(lines)
        .unwrap()
        .0
        .patch(1, 0, Cell::Fixed(2));

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

    let lines = BufReader::new(puzzle_str.as_bytes()).lines();
    
    let puzzle = Board::parse_puzzle(lines);
    assert_eq!(puzzle_str, puzzle.unwrap().0.to_string());
}

#[test]
fn test_coords() {
    assert_eq!((1, 0), next_coords(0, 0));
    assert_eq!((0, 1), next_coords(8, 0));
}

#[test]
fn test_get_possible() {
    let lines = BufReader::new(" 23456789
2        
3        
4        
5        
6        
7        
8        
9        ".as_bytes()).lines();

    let board = Board::parse_puzzle(lines).unwrap().0;

    let expected: HashSet<u8> = vec![1].into_iter().collect();
    assert_eq!(expected, board.get_possible(0, 0));
}
