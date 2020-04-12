pub mod tests;

use std::num::NonZeroU8;

#[derive(Debug)]
pub struct Board {
    // TODO: 
    // - store array of 9*9 = 27 pieces
    pub tiles : [[Option<NonZeroU8>; 9]; 9],
}


impl Board {
    // TODO initializers: from 9*9 array, from 9*9 w/ 0 instead of None?, from sparse mapping?

}
