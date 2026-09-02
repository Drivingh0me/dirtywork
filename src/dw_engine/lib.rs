mod mcmath;

use error::Result;

pub enum Color {
    white,
    black,
}

pub enum Checkmate {
    no,
    yes(winner: Color)
}

// Bitboard is a1 -> h1 -> a2 -> h2... -> h8
struct piece {
    bit_board: u64,
}

pub struct BoardState {
    tomove: Color,
    w_pawn: piece,
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

impl BoardState {
    fn new() -> Self {

    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            haswon: no,
            incheck: false,
            board: Vec::<BoardState>::new();
        }
    }
}

pub struct GameState {
    haswon: Checkmate,
    incheck: bool,
    board: Vec<BoardState>,
    enpassant: piece,
}

pub fn dw_analysis(
    state: GameState,
    depth: u32
) -> Result<(best_move: BoardState, eval: f32)> {
    println!("ran analysis");
}
