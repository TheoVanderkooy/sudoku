use sudoku::*;


fn main() -> Result<(), SudokuError> {

    // debug printing
    // println!("{}\n\n", Board::empty());

    let mut board = Board::from_sparse(&vec![(0, 0, 1), (2, 3, 2), (8, 8, 3)])?;
    // println!("{}\n\n", board);

    board[1][1].set(4)?;
    println!("{}\n\n", board);
    board[1][1].set(0)?;
    println!("\n\n{}", board);


    Ok(())
}
