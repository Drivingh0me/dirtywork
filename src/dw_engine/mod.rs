mod mcmath;
// pub mod ui;

use crate::error::{Result, Error};

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Color {
    #[default]
    White,
    Black,
}

// Bitboard is a1 -> h1 -> a2 -> h2... -> h8
#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Piece {
    pub bit_board: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoardState {
    pub tomove: Color,
    pub w_pawn: Piece,
    pub w_knight: Piece,
    pub w_bishop: Piece,
    pub w_rook: Piece,
    pub w_queen: Piece,
    pub w_king: Piece,
    pub b_pawn: Piece,
    pub b_knight: Piece,
    pub b_bishop: Piece,
    pub b_rook: Piece,
    pub b_queen: Piece,
    pub b_king: Piece,
    // Pawn that moved two squares in the previous turn.
    pub dbl_pawn: Piece,
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
            w_pawn: Piece { bit_board: 65280 },
            w_knight: Piece { bit_board: 66 },
            w_bishop: Piece { bit_board: 36 },
            w_rook: Piece { bit_board: 129 },
            w_queen: Piece { bit_board: 16 },
            w_king: Piece { bit_board: 8 },
            b_pawn: Piece { bit_board: 71776119061217280 },
            b_knight: Piece { bit_board: 4755801206503243776 },
            b_bishop: Piece { bit_board: 2594073385365405696 },
            b_rook: Piece { bit_board: 9295429630892703744 },
            b_queen: Piece { bit_board: 1152921504606846976 },
            b_king: Piece { bit_board: 576460752303423488 },
            dbl_pawn: Piece { bit_board: 0 },
        }
    }
}

pub struct GameState {
    haswon: Option<Color>,
    incheck: bool,
    board: Vec<BoardState>,
}

impl Default for GameState {
    fn default() -> Self {
        let mut board = Vec::<BoardState>::new();

        let mut starting_position = BoardState::default();

        board.push(starting_position);

        Self {
            haswon: Option::default(),
            incheck: false,
            board: board,
        }
    }
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
