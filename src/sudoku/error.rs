
use std::fmt;


#[derive(Debug)]
pub struct SudokuError {
    pub msg: String,
}

impl std::error::Error for SudokuError { }

impl fmt::Display for SudokuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SudokuError: {}", self.msg)
    }
}