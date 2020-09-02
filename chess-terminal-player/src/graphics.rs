#[allow(dead_code)]
pub mod table {
    pub const BOTTOM_RIGHT_CORNER: &'static str = "╝";
    pub const BOTTOM_LEFT_CORNER: &'static str = "╚";
    pub const TOP_RIGHT_CORNER: &'static str = "╗";
    pub const TOP_LEFT_CORNER: &'static str = "╔";
    pub const HORIZONTAL_WALL: &'static str = "═";
    pub const VERTICAL_WALL: &'static str = "║";
    pub const HORIZONTAL_LEFT_DT: &'static str = "╠";
    pub const HORIZONTAL_RIGHT_DT: &'static str = "╣";
    pub const VERTICAL_BOTTOM_DT: &'static str = "╩";
    pub const VERTICAL_TOP_DT: &'static str = "╦";

    pub const HORIZONTAL_LINE: &'static str = "─";
    pub const VERTICAL_LINE: &'static str = "│";
    pub const CROSS: &'static str = "┼";

    pub const HORIZONTAL_LEFT_T: &'static str = "╟";
    pub const HORIZONTAL_RIGHT_T: &'static str = "╢";
    pub const VERTICAL_BOTTOM_T: &'static str = "╧";
    pub const VERTICAL_TOP_T: &'static str = "╤";
}
#[allow(dead_code)]
pub mod pieces {
    pub const WHITE_KING: &'static str = "♔";
    pub const WHITE_QUEEN: &'static str = "♕";
    pub const WHITE_ROOK: &'static str = "♖";
    pub const WHITE_BISHOP: &'static str = "♗";
    pub const WHITE_KNIGHT: &'static str = "♘";
    pub const WHITE_PAWN: &'static str = "♙";

    pub const BLACK_KING: &'static str = "♚";
    pub const BLACK_QUEEN: &'static str = "♛";
    pub const BLACK_ROOK: &'static str = "♜";
    pub const BLACK_BISHOP: &'static str = "♝";
    pub const BLACK_KNIGHT: &'static str = "♞";
    pub const BLACK_PAWN: &'static str = "♟";
}
#[allow(dead_code)]
pub mod borders {
    pub const TOP: &'static str = "╔═══╤═══╤═══╤═══╤═══╤═══╤═══╤═══╗";
    pub const BOTTOM: &'static str = "╚═══╧═══╧═══╧═══╧═══╧═══╧═══╧═══╝";
    pub const MID: &'static str = "╟───┼───┼───┼───┼───┼───┼───┼───╢";
}
