mod sudoku;

mod tests;

use sudoku::Board;



fn main() {

    let empty_board = Board::empty();
    
    // let b2 = Board{tiles: [
    //     [std::num::NonZeroU8::new(1),    None,    None,    None,    None,    None,    None,    None,    None   ],
    //     [None,    std::num::NonZeroU8::new(2),    None,    None,    None,    None,    None,    None,    None   ],
    //     [None,    None,    std::num::NonZeroU8::new(3),    None,    None,    None,    None,    None,    None   ],
    //     [None,    None,    None,    std::num::NonZeroU8::new(4),    None,    None,    None,    None,    None   ],
    //     [None,    None,    None,    None,    std::num::NonZeroU8::new(5),    None,    None,    None,    None   ],
    //     [None,    None,    None,    None,    None,    std::num::NonZeroU8::new(6),    None,    None,    None   ],
    //     [None,    None,    None,    None,    None,    None,    std::num::NonZeroU8::new(7),    None,    None   ],
    //     [None,    None,    None,    None,    None,    None,    None,    std::num::NonZeroU8::new(8),    None   ],
    //     [None,    None,    None,    None,    None,    None,    None,    None,    std::num::NonZeroU8::new(9)   ],
    // ]};

    let _x: u8 = 3;


    println!("{}", empty_board);
    // println!("{}", b2);
}
