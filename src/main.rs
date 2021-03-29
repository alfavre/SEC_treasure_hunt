mod board;
use board::Board;


fn main() {
    println!("The one piece is the friends we made along the way");
    let my_board = Board::new(12);
    my_board.print();
    print!("board -> {:?} \n",my_board);
}
