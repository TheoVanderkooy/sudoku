use super::Board;

#[derive(Debug)]
pub struct SudokuGame {
    board: Board,
}


impl SudokuGame {
    // TODO new game function (equivalent to start?)

    pub fn new(board: Board) -> Self {
        SudokuGame{board}
    }

    

    pub fn start(&mut self) {
        self.board.reset();

        println!("New game!");
        println!("{}\n", self.board);
    }


}