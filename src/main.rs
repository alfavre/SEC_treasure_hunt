mod board;
use board::*;

/// The main function for our treasure search game
///
/// All non-game related manipulation are put here
fn main() {
    println!("The one piece is the friends we made along the way");
    match Board::play_game() {
        Ok(_) => println!("executed withour errors"),
        Err(_) => println!("game returned an error"),
    }
}
