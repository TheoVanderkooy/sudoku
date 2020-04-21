
use super::SudokuError;

use std::num::NonZeroU8;


// TODO better doc comments
// TODO tests?

/// A tile in a sudoku grid.
#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Fixed(NonZeroU8),
    Variable(Option<NonZeroU8>),
}


impl Tile {
    /// Returns a new fixed-value tile with the given value.
    /// Panics if `val` is not between 1 and 9 (inclusive).
    pub fn fixed(val: u8) -> Tile {
        match NonZeroU8::new(val) {
            Some(x) if x.get() <= 9u8 => Tile::Fixed(x),
            _ => panic!("Invalid value for fixed tile."),
        }
    }

    /// Returns a new empty variable tile.
    pub fn variable() -> Tile {
        Tile::Variable(Option::None)
    }

    pub fn get(&self) -> Option<NonZeroU8> {
        match self {
            Tile::Fixed(x) => Option::Some(x.clone()),
            Tile::Variable(x) => x.clone(),
        }
    }

    /// ```
    /// let mut board = sudoku::Board::empty();
    /// board[0][1].set(3);
    /// assert!(board[0][1].get().filter(|v| v.get() == 3).is_some());
    /// ```
    pub fn set(&mut self, val: u8) -> Result<(), SudokuError> {
        if val > 9 {
            return Err(SudokuError{msg: format!("Invalid value for tile {}", val)});
        }

        match self {
            Tile::Fixed(_) => Err(SudokuError{msg: "Cannot set value for a fixed value tile".to_string()}),
            Tile::Variable(ref mut x) => {
                *x = NonZeroU8::new(val);
                Ok(())
            }
        }
    }
}