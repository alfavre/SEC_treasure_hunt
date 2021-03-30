mod board;
use board::Board;

/// The main function for our treasure search game
/// 
/// All non-game related manipulation are put here
fn main() {
    println!("The one piece is the friends we made along the way");
    let my_board = Board::new(2);
    my_board.print();
    print!("board -> {:?} \n", my_board);
}
