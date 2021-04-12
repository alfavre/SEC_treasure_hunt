mod board;
use board::*;
use read_input::prelude::*;

/// The main function for our treasure search game
///
/// All non-game related manipulation are put here
fn main() {
    println!("The one piece is the friends we made along the way");
    Board::play_game();
    // let my_board = Board::new(2);
    //my_board.print();
}
