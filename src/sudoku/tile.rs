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

    // TODO function to set value - should fail on fixed tiles - only valid values for variable
}