use std::char::from_digit;
use std::io::{Read, BufReader, BufRead};

#[derive(Debug, Copy, Clone, PartialEq)]
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

struct Board {
    grid: [[Cell; 9]; 9]
}

impl Board {
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

        grid[x][y] = cell;
        
        Board {
            grid: grid,
        }
    }

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
    
}


#[cfg(test)]
#[test]
fn test_patch() {
    let board = Board::new(
        vec!((0, 0, 1))
    ).patch(0, 1, Cell::Fixed(2));

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
    println!(":\n{}\n", puzzle_str);
    println!(":\n{}\n", puzzle.to_string());
    assert_eq!(puzzle_str, puzzle.to_string());
}
        
