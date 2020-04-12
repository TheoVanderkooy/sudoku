mod sudoku_board; 



fn main() {
    let empty_board = sudoku_board::Board{tiles: [
        [None,None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None,None],
    ]};
    
    println!("Hello, world!");
}
