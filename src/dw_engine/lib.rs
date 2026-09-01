mod mcmath;

use error::Result;

enum color {
    white,
    black,
}

// Bitboard is a1 -> h1 -> a2 -> h2... -> h8
struct piece {
    bit_board: u64,
}

struct BoardState {
    tomove: color,
    haswon: color,
    w_pawn: piece
    w_knight: piece,
    w_bishop: piece,
    w_rook: piece,
    w_queen: piece,
    w_king: piece,
    b_pawn: piece,
    b_knight: piece,
    b_bishop: piece,
    b_rook: piece,
    b_queen: piece,
    b_king: piece,
}

pub fn dw_analysis(state: BoardState, depth: u32) -> Result<()> {
    println!("ran analysis");
}
