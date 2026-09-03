mod mcmath;

use crate::error::Result;
use crate::error::Error;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Color {
    #[default]
    White,
    Black,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Checkmate {
    #[default]
    No,
    Yes(Color)
}

// Bitboard is a1 -> h1 -> a2 -> h2... -> h8
#[derive(Debug, PartialEq, Default, Clone, Copy)]
struct Piece {
    bit_board: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoardState {
    tomove: Color,
    w_pawn: Piece,
    w_knight: Piece,
    w_bishop: Piece,
    w_rook: Piece,
    w_queen: Piece,
    w_king: Piece,
    b_pawn: Piece,
    b_knight: Piece,
    b_bishop: Piece,
    b_rook: Piece,
    b_queen: Piece,
    b_king: Piece,
    // Pawn that moved two squares in the previous turn.
    dbl_pawn: Piece,
}

impl BoardState {
    fn new() -> Self {
        Self {
            tomove: Color::White,
            w_pawn: Piece { bit_board: 0 },
            w_knight: Piece { bit_board: 0 },
            w_bishop: Piece { bit_board: 0 },
            w_rook: Piece { bit_board: 0 },
            w_queen: Piece { bit_board: 0 },
            w_king: Piece { bit_board: 0 },
            b_pawn: Piece { bit_board: 0 },
            b_knight: Piece { bit_board: 0 },
            b_bishop: Piece { bit_board: 0 },
            b_rook: Piece { bit_board: 0 },
            b_queen: Piece { bit_board: 0 },
            b_king: Piece { bit_board: 0 },
            dbl_pawn: Piece { bit_board: 0 },
        }
    }
}

impl Default for BoardState {
    fn default() -> Self {
        // Starting position for all pieces.
        Self{
            tomove: Color::White,
            w_pawn: Piece { bit_board: 0 },
            w_knight: Piece { bit_board: 0 },
            w_bishop: Piece { bit_board: 0 },
            w_rook: Piece { bit_board: 0 },
            w_queen: Piece { bit_board: 0 },
            w_king: Piece { bit_board: 0 },
            b_pawn: Piece { bit_board: 0 },
            b_knight: Piece { bit_board: 0 },
            b_bishop: Piece { bit_board: 0 },
            b_rook: Piece { bit_board: 0 },
            b_queen: Piece { bit_board: 0 },
            b_king: Piece { bit_board: 0 },
            dbl_pawn: Piece { bit_board: 0 },
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        let mut board = Vec::<BoardState>::new();

        let mut starting_position = BoardState::default();

        board.push(starting_position);

        Self {
            haswon: Checkmate::No,
            incheck: false,
            board: board,
        }
    }
}

pub struct GameState {
    haswon: Checkmate,
    incheck: bool,
    board: Vec<BoardState>,
}

// Returns the best move as a BoardState and the eval as a float.
pub fn dw_analysis(
    state: GameState,
    depth: u32
) -> Result<(BoardState, f32)> {
    let eval: f32 = 0.0;
    let best_move: BoardState = state
        .board
        .last()
        .cloned()
        .ok_or(
        Error::VectorSize
    )?;

    println!("ran analysis");
    Ok((best_move, eval))
}
