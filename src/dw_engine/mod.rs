mod mcmath;
// pub mod ui;

use crate::error::{Result, Error};

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Color {
    #[default]
    White,
    Black,
}

pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

pub struct Move {
    piece: Piece,
    bits: BitBoard,
}

// Bitboard is a1 -> h1 -> a2 -> h2... -> h8
#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct BitBoard {
    bits: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoardState {
    tomove: Color,
    w_pawn: BitBoard,
    w_knight: BitBoard,
    w_bishop: BitBoard,
    w_rook: BitBoard,
    w_queen: BitBoard,
    w_king: BitBoard,
    b_pawn: BitBoard,
    b_knight: BitBoard,
    b_bishop: BitBoard,
    b_rook: BitBoard,
    b_queen: BitBoard,
    b_king: BitBoard,
    // Pawn that moved two squares in the previous turn.
    dbl_pawn: BitBoard,
}

impl BoardState {
    fn new() -> Self {
        Self {
            tomove: Color::White,
            w_pawn: BitBoard { bits: 0 },
            w_knight: BitBoard { bits: 0 },
            w_bishop: BitBoard { bits: 0 },
            w_rook: BitBoard { bits: 0 },
            w_queen: BitBoard { bits: 0 },
            w_king: BitBoard { bits: 0 },
            b_pawn: BitBoard { bits: 0 },
            b_knight: BitBoard { bits: 0 },
            b_bishop: BitBoard { bits: 0 },
            b_rook: BitBoard { bits: 0 },
            b_queen: BitBoard { bits: 0 },
            b_king: BitBoard { bits: 0 },
            dbl_pawn: BitBoard { bits: 0 },
        }
    }
}

impl Default for BoardState {
    fn default() -> Self {
        // Starting position for all pieces.
        Self{
            tomove: Color::White,
            w_pawn: BitBoard { bits: 65280 },
            w_knight: BitBoard { bits: 66 },
            w_bishop: BitBoard { bits: 36 },
            w_rook: BitBoard { bits: 129 },
            w_queen: BitBoard { bits: 16 },
            w_king: BitBoard { bits: 8 },
            b_pawn: BitBoard { bits: 71776119061217280 },
            b_knight: BitBoard { bits: 4755801206503243776 },
            b_bishop: BitBoard { bits: 2594073385365405696 },
            b_rook: BitBoard { bits: 9295429630892703744 },
            b_queen: BitBoard { bits: 1152921504606846976 },
            b_king: BitBoard { bits: 576460752303423488 },
            dbl_pawn: BitBoard { bits: 0 },
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
    depth: usize
) -> Result<(BoardState, f32)> {
    println!("thinking...");

    // Loop over this. ------
    // Make a legal move.
    let pos_eval = evaluate_pos(state
        .board
        .last()
        // .cloned()
        .ok_or(Error::VectorSize)?
    );

    //------------------------

    let eval: f32 = 0.0;
    let best_move: BoardState = state
        .board
        .last()
        .cloned()
        .ok_or(Error::VectorSize)?;

    println!("ran analysis");
    Ok((best_move, eval))
}

fn evaluate_pos(state: &BoardState) -> f32 {
    let material_weight: f32 = 0.5;
    let control_weight: f32 = 0.3;
    let agro_weight: f32 = 0.2;

    let material = count_material(&state);
    println!("material is: {}", material);

    let control = measure_control(&state);
    println!("control is: {}", control);

    0.0
}

fn count_material(state: &BoardState) -> f32 {
    let p = how_many(state.w_pawn) - how_many(state.b_pawn);
    let n = how_many(state.w_knight) - how_many(state.b_knight);
    let b = how_many(state.w_bishop) - how_many(state.b_bishop);
    let r = how_many(state.w_rook) - how_many(state.b_rook);
    let q = how_many(state.w_queen) - how_many(state.b_queen);

    let p = p as f32;
    let n = n as f32;
    let b = b as f32;
    let r = r as f32;
    let q = q as f32;

    1.0 * p + 2.9 * n + 3.0 * b + 5.0 * r + 9.0 * q
}

fn how_many(p: BitBoard) -> u8 {
    let mut bits = p.bits;
    let mut num: u8 = 0;
    while bits != 0 {
        bits &= bits - 1;
        num += 1;
    }
    num
}

fn measure_control(state: &BoardState) -> f32 {
    // Form a movemap for all 64 positions and xor the current map.

    0.0
}

pub fn get_piece_coords(board: &BoardState, piece: &Piece) -> Vec<(u8, u8)> {
    let bitboard = get_bitboard(&board, piece);

    let mut pieces = Vec::new();
    let mut bb = bitboard.bits;

    // AI generated, human verified--------.
    while bb != 0 {
        // Get the index of the lowest set bit (0-63)
        let square = bb.trailing_zeros() as u8;

        // Convert index to coordinates (file and rank)
        let file = square % 8;
        let rank = square / 8;
        pieces.push((file, rank));
        // Clear the lowest set bit
        bb &= bb - 1;
    }
    //--------------------------------------.

    pieces
}

fn get_bitboard(board: &BoardState, piece: &Piece) -> BitBoard {
    let bitboard: BitBoard = match piece {
        Piece::Pawn(color) => match color {
            Color::White => board.w_pawn,
            Color::Black => board.b_pawn,
        }
        Piece::Knight(color) => match color {
            Color::White => board.w_knight,
            Color::Black => board.b_knight,
        }
        Piece::Bishop(color) => match color {
            Color::White => board.w_bishop,
            Color::Black => board.b_bishop,
        }
        Piece::Rook(color) => match color {
            Color::White => board.w_rook,
            Color::Black => board.b_rook,
        }
        Piece::Queen(color) => match color {
            Color::White => board.w_queen,
            Color::Black => board.b_queen,
        }
        Piece::King(color) => match color {
            Color::White => board.w_king,
            Color::Black => board.b_king,
        }
        _ => board.dbl_pawn,
    };

    bitboard

}
