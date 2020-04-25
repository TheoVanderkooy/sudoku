use sudoku::{Board, SudokuError, SudokuGame};



fn main() -> Result<(), SudokuError> {
    let board = Board::from_sparse(&vec![(0, 0, 1), (2, 3, 2), (8, 8, 3)])?;

    let mut game = SudokuGame::new(board);
    game.start();

    Ok(())
}
