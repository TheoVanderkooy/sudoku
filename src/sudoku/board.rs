use super::Tile;
use super::SudokuError;

use std::ops::{Index, IndexMut};


#[derive(Debug)]
pub struct Board {
    pub tiles : [[Tile; 9]; 9],
}


impl Board {
    // TODO initializers: from 9*9 array, from 9*9 w/ 0 instead of None?, from sparse mapping?

    /// Create an empty board 
    pub fn empty() -> Board {
        Board{tiles: [[Tile::variable(); 9]; 9]}
    }

    pub fn from_sparse(fixed_tiles: &Vec<(u8, u8, u8)>) -> Result<Board, SudokuError> {
        let mut board = Board::empty();

        for (x, y, v) in fixed_tiles {
            if  x >= &9 {
                return Err(SudokuError{msg: format!("invalid x value {}", x)});
            } else if y >= &9 {
                return Err(SudokuError{msg: format!("invalid y value {}", y)});
            } else if v < &1 || v > &9 {
                return Err(SudokuError{msg: format!("invalid tile value {}", v)});
            }

            let t = &mut board[*y][*x as usize];
            match *t {
                Tile::Fixed(_) => return Err(SudokuError{msg: format!("Multiple values specified for file ({}, {})", x, y)}),
                Tile::Variable(_) => *t = Tile::fixed(*v),
            }
        }

        Ok(board)
    }

}

// Print out the board
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        
        for i in 0..9 {
            // Print horizontal separators between boxes
            if i == 3 || i == 6 {
                f.write_str("\n-------+-------+-------\n")?
            } else if i != 0 {
                f.write_str("\n")?
            }

            // Print the line
            for j in 0..9 {
                if j == 3 || j == 6 {
                    f.write_str(" |")?
                }
                match self.tiles[i][j].get() {
                    Some(x) => f.write_fmt(format_args!(" {}", x.to_string())),
                    None => f.write_str("  ")
                }?
            }
        }
        
        std::result::Result::Ok(())
    }
}

impl Index<u8> for Board {
    type Output = [Tile; 9];

    fn index(&self, index: u8) -> &Self::Output {
        &self.tiles[index as usize]
    }
}

impl IndexMut<u8> for Board {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.tiles[index as usize]
    }
}