mod sudoku;

mod tests;

use sudoku::*;



fn main() -> Result<(), SudokuError> {

    let empty_board = Board::empty();


    println!("{}", empty_board);

    let board = Board::from_sparse(&vec![(0, 0, 1), (2, 3, 2), (8, 8, 3)])?;

    println!("{}", board);

    Ok(())
}
