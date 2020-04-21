
#[cfg(test)]

use ::sudoku::{Board};



#[test]
fn test_print_board() {
    let empty_board_str = Board::empty();
    let formated = format!("{}", empty_board_str);

    assert_eq!(formated, "       |       |      \n       |       |      \n       |       |      \n-------+-------+-------\n       |       |      \n       |       |      \n       |       |      \n-------+-------+-------\n       |       |      \n       |       |      \n       |       |      ");

    // TODO get an actual board and test that

}
