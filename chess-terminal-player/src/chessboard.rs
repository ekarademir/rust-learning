use std::fmt;
use std::collections::HashMap;

extern crate termion;
extern crate regex;

use termion::{
    clear,
    color,
    cursor,
};

use regex::Regex;

#[path = "graphics.rs"]
mod graphics;

use graphics::{
    borders,
    pieces as p,
    table,
};

const SPACE: &'static str = " ";

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum PieceKind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
    None,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Colour {
    Black,
    White,
    None,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Piece {
    kind: PieceKind,
    colour: Colour,
}

impl Piece {
    fn new(kind: PieceKind, colour: Colour) -> Piece {
        Piece {kind, colour}
    }

    fn terminal_character(&self) -> String {
        let a = match (&self.colour, &self.kind) {
            (Colour::Black, PieceKind::Pawn) => p::BLACK_PAWN,
            (Colour::Black, PieceKind::Bishop) => p::BLACK_BISHOP,
            (Colour::Black, PieceKind::Knight) => p::BLACK_KNIGHT,
            (Colour::Black, PieceKind::Rook) => p::BLACK_ROOK,
            (Colour::Black, PieceKind::Queen) => p::BLACK_QUEEN,
            (Colour::Black, PieceKind::King) => p::BLACK_KING,
            (Colour::White, PieceKind::Pawn) => p::WHITE_PAWN,
            (Colour::White, PieceKind::Bishop) => p::WHITE_BISHOP,
            (Colour::White, PieceKind::Knight) => p::WHITE_KNIGHT,
            (Colour::White, PieceKind::Rook) => p::WHITE_ROOK,
            (Colour::White, PieceKind::Queen) => p::WHITE_QUEEN,
            (Colour::White, PieceKind::King) => p::WHITE_KING,
            _ => " "
        };
        String::from(a)
    }

    fn render(&self) -> String {
        format!("{}", self.terminal_character())
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

struct Tile { }

impl Tile {
    fn render(row: usize, col:char, piece: &Option<Piece>) -> Result<String, String> {
        if !['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].contains(&col) {
            return Err(format!("Invalid column, {}", col));
        }
        if !(0 < row && row < 9) {
            return Err(format!("Invalid row, {}", row));
        }

        let rendered_piece = match piece {
            Some(p) => p.render(),
            None => String::from(SPACE),
        };

        let black_start_cols = vec!['a', 'c', 'e', 'g'];
        let starts_with_black = black_start_cols.contains(&col);
        let row_is_even = row % 2 == 0;

        match (starts_with_black, row_is_even) {
            (true, true) => Ok(format!("{}{} {} {}", color::Bg(color::Reset), color::Bg(color::White), rendered_piece, color::Bg(color::Reset))),
            (true, false) => Ok(format!("{}{} {} {}", color::Bg(color::Reset), color::Bg(color::Black), rendered_piece, color::Bg(color::Reset))),
            (false, true) => Ok(format!("{}{} {} {}", color::Bg(color::Reset), color::Bg(color::Black), rendered_piece, color::Bg(color::Reset))),
            (false, false) => Ok(format!("{}{} {} {}", color::Bg(color::Reset), color::Bg(color::White), rendered_piece, color::Bg(color::Reset))),
        }
    }
}

enum Actions {
    Moves,
    Takes,
    Promoted,
    Castled,
}

#[derive(Debug)]
enum Instructions {
    Add(Piece, (usize, usize)),
    Remove((usize, usize)),
}

pub struct ChessBoard {
    // Two mappings, one from board to pieces, the other from pieces to board.
    boardstate: [[Piece; 8]; 8],  // Outer array for rows, inner array for columns BoardState[0][0] is a1; BoardState[7][7] is h8
    piecemap : HashMap<(Piece), Vec<(usize, usize)> >,  // (PIECETYPE, NUMBER) -> (ROW, COL)
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut pm = HashMap::new();
        pm.insert(Piece::new(PieceKind::Rook, Colour::White), vec![(0, 0), (0, 7)]);
        pm.insert(Piece::new(PieceKind::Knight, Colour::White), vec![(0, 1), (0, 6)]);
        pm.insert(Piece::new(PieceKind::Bishop, Colour::White), vec![(0, 2), (0, 5)]);
        pm.insert(Piece::new(PieceKind::Queen, Colour::White), vec![(0, 3)]);
        pm.insert(Piece::new(PieceKind::King, Colour::White), vec![(0, 4)]);
        pm.insert(Piece::new(PieceKind::Rook, Colour::Black), vec![(0, 0), (0, 7)]);
        pm.insert(Piece::new(PieceKind::Knight, Colour::Black), vec![(0, 1), (0, 6)]);
        pm.insert(Piece::new(PieceKind::Bishop, Colour::Black), vec![(0, 2), (0, 5)]);
        pm.insert(Piece::new(PieceKind::Queen, Colour::Black), vec![(0, 3)]);
        pm.insert(Piece::new(PieceKind::King, Colour::Black), vec![(0, 4)]);

        let white_pawn = Piece::new(PieceKind::Pawn, Colour::White);
        let black_pawn = Piece::new(PieceKind::Pawn, Colour::Black);


        for i in 0..8 {
            let w = pm.entry(white_pawn).or_insert(Vec::new());
            w.push((1, i));
            let b = pm.entry(black_pawn).or_insert(Vec::new());
            b.push((7, i));
        }

        let state = [
            [
                Piece::new(PieceKind::Rook, Colour::White),     // a1
                Piece::new(PieceKind::Knight, Colour::White),
                Piece::new(PieceKind::Bishop, Colour::White),
                Piece::new(PieceKind::Queen, Colour::White),
                Piece::new(PieceKind::King, Colour::White),
                Piece::new(PieceKind::Bishop, Colour::White),
                Piece::new(PieceKind::Knight, Colour::White),
                Piece::new(PieceKind::Rook, Colour::White),     // h1
            ],
            [Piece::new(PieceKind::Pawn, Colour::White); 8],
            [Piece::new(PieceKind::None, Colour::None); 8],
            [Piece::new(PieceKind::None, Colour::None); 8],
            [Piece::new(PieceKind::None, Colour::None); 8],
            [Piece::new(PieceKind::None, Colour::None); 8],
            [Piece::new(PieceKind::Pawn, Colour::Black); 8],
            [
                Piece::new(PieceKind::Rook, Colour::Black),     // a8
                Piece::new(PieceKind::Knight, Colour::Black),
                Piece::new(PieceKind::Bishop, Colour::Black),
                Piece::new(PieceKind::Queen, Colour::Black),
                Piece::new(PieceKind::King, Colour::Black),
                Piece::new(PieceKind::Bishop, Colour::Black),
                Piece::new(PieceKind::Knight, Colour::Black),
                Piece::new(PieceKind::Rook, Colour::Black),     // h8
            ]
        ];

        ChessBoard {
            boardstate: state,
            piecemap: ChessBoard::build_piecemap(state),
        }
    }

    fn build_piecemap(boardstate: [[Piece; 8]; 8]) -> HashMap<(Piece), Vec<(usize, usize)> > {
        let mut pm = HashMap::new();
        for (row, cols) in boardstate.iter().enumerate() {
            for (col, piece) in cols.iter().enumerate() {
                if piece.kind == PieceKind::None {continue;}
                let list = pm.entry(*piece).or_insert(Vec::new());
                list.push((row, col));
            }
        }
        pm
    }

    pub fn draw(&self) {
        let rows = [1, 2, 3, 4, 5, 6, 7, 8];
        let cols = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        // Clear screen
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        for (row, pieces) in self.boardstate.iter().enumerate().rev() {  // a1 is at the bottom
            // Horizontal lines
            if row == 7 { println!(" {}", borders::TOP); }
            else {println!(" {}", borders::MID)}

            for (col, piece) in pieces.iter().enumerate() {
                // Vertical Lines
                if col == 0 { print!("{}{}", rows[row], table::VERTICAL_WALL) }
                else { print!("{}", table::VERTICAL_LINE) }

                match Tile::render(rows[row], cols[col], &Some(*piece)) {
                    Ok(tile) => print!("{}", tile),
                    Err(msg) => print!("{}", msg),
                }
            }
            println!("{}", table::VERTICAL_WALL);
        }
        println!(" {}", borders::BOTTOM);
        // Column letters
        print!("  ");
        for col in cols.iter() {
            print!(" {}  ", col);
        }
        println!();
    }

    pub fn move_piece(&mut self, movetext: &'static str) {
        // for x in &self.piecemap {
        //     println!("BEFORE {:?}", x);
        // }
        match self.parse_piece_move(movetext, Colour::White) {
            Some(instructions) =>  {
                for inst in instructions {
                    match inst {
                        Instructions::Add(piece, pos) => {
                            self.boardstate[pos.0][pos.1] = piece
                        },
                        Instructions::Remove(pos) => {
                            self.boardstate[pos.0][pos.1] = Piece::new(PieceKind::None, Colour::None)
                        },
                    }
                }
            },
            None => {}
        }
        self.piecemap = ChessBoard::build_piecemap(self.boardstate);
        // for x in &self.piecemap {
        //     println!("AFTER {:?}", x);
        // }
    }

    fn parse_piece_move(&self, x: &'static str, color: Colour) -> Option<Vec<Instructions>> {
        // So far only moves, no captures, no disambiguation.
        let m = x.trim();
        match Regex::new(r"([BKNQR]*)([abcdefgh])([12345678])") {
            Ok(re) => {
                match re.captures(m) {
                    Some(capture) => {
                        // Match regex results.
                        let piece_kind = match &capture[1] {
                            "B" => PieceKind::Bishop,
                            "K" => PieceKind::King,
                            "N" => PieceKind::Knight,
                            "Q" => PieceKind::Queen,
                            "R" => PieceKind::Rook,
                            "" => PieceKind::Pawn,
                            _ => PieceKind::None,
                        };
                        let col: usize = match &capture[2] {
                            "a" => 0,
                            "b" => 1,
                            "c" => 2,
                            "d" => 3,
                            "e" => 4,
                            "f" => 5,
                            "g" => 6,
                            "h" => 7,
                            _ => 8,
                        };
                        let row = match &capture[3] {
                            x if ["1", "2", "3", "4", "5", "6", "7", "8"].contains(&x) => match x.parse::<usize>() {
                                Ok(n) => n - 1,
                                Err(_) => 8,
                            },
                            _ => 8,
                        };
                        if piece_kind != PieceKind::None && col < 8 && row < 8 {
                            return self.compile_instructions(piece_kind, color, (row, col), Actions::Moves).ok();
                        }
                        None
                    },
                    None => None,
                }
            },
            Err(_) => None,
        }
    }

    fn compile_instructions(&self, piece_kind: PieceKind, colour: Colour, destination: (usize, usize), action: Actions) -> Result<Vec<Instructions>, &'static str> {
        let piece = Piece::new(piece_kind, colour);
        match self.piecemap.get(&piece) {
            Some(v) => {
                for pos in v { // Go through all pieces that matches the type and find which can perform the action. There should only be one.
                    if self.verify_action(&pos, &destination, &action) {
                        return Ok(vec![
                            Instructions::Add(piece, destination),
                            Instructions::Remove(*pos)
                        ]);
                    }
                }
                Err("Invalid move")
            },
            None => Err("Can't find the piece."),
        }
    }

    fn verify_action(&self, from: &(usize, usize), to: &(usize, usize), action: &Actions) -> bool {
        // from is always guarantied to have a piece
        let piece = self.boardstate[from.0][from.1];
        match piece.kind {
            PieceKind::Pawn => {
                match action {
                    Actions::Moves => {
                        if from.1 == to.1 {  // If pawn moves, it should be on the same column.
                            true
                        } else {
                            false
                        }
                    }
                    _ => false
                }
            },
            _ => false
        }
    }
}
