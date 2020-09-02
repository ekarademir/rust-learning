extern crate termion;

use termion::terminal_size;

mod chessboard;
use chessboard::ChessBoard;

fn main() {
    let mut board = match terminal_size() {
        Ok(_) => ChessBoard::new(),
        _ => panic!("Can't get terminal size!")
    };
    board.move_piece("e4");
    board.draw();
}
