use super::Tile;

use std::num::NonZeroU8;


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