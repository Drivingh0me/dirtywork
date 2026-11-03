mod boards;
mod magic;
mod pieces;

pub use magic::{initialize_magic, search_magic};
pub use boards::{BitBoard, BoardState};
pub use pieces::{PieceType, Color};
use crate::dw_engine::magic::Magic;
use crate::dw_engine::pieces::{Moov};
use crate::dw_engine::boards::{Square};

use crate::error::{Result, Error};

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
            haswon: Option::None,
            incheck: false,
            board: board,
        }
    }
}

// Need to make this continuously return according to UCI standard, and can
// stop. Make it return depth, score, etc  that can be parsed to
// form a UCI standard stdout through UI.
pub fn dw_analysis(
    state: GameState,
    depth: usize,
    magic: Magic,
) -> Result<(BoardState, f32)> {
    println!("thinking...");

    // Loop over this depth times. ------
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

    println!("ran analysis!");
    Ok((best_move, eval))
}

// Static evaluation.
fn evaluate_pos(state: &BoardState) -> f32 {
    let material_weight: f32 = 0.5;
    let control_weight: f32 = 0.3;
    let agro_weight: f32 = 0.2;

    let material = count_material(&state);
    println!("material is: {}", material);

    let control = measure_control(&state);
    println!("control is: {}", control);

    let agro = aggression(&state);
    println!("aggression is: {}", agro);

    (material_weight * material +
    control_weight * control +
    agro_weight * agro)
}

fn aggression(state: &BoardState) -> f32 {
    // See how close the average piece is to the king.

    0.0
}

fn count_material(state: &BoardState) -> f32 {
    let p = how_many(state.get_bb(PieceType::Pawn, Color::White))
        - how_many(state.get_bb(PieceType::Pawn, Color::Black));
    let n = how_many(state.get_bb(PieceType::Knight, Color::White))
        - how_many(state.get_bb(PieceType::Knight, Color::Black));
    let b = how_many(state.get_bb(PieceType::Bishop, Color::White))
        - how_many(state.get_bb(PieceType::Bishop, Color::Black));
    let r = how_many(state.get_bb(PieceType::Rook, Color::White))
        - how_many(state.get_bb(PieceType::Rook, Color::Black));
    let q = how_many(state.get_bb(PieceType::Queen, Color::White))
        - how_many(state.get_bb(PieceType::Queen, Color::Black));

    let p = p as f32;
    let n = n as f32;
    let b = b as f32;
    let r = r as f32;
    let q = q as f32;

    1.0 * p + 3.0 * n + 3.0 * b + 5.0 * r + 9.0 * q
}

// Calculates how many pieces there are.
fn how_many(piece: BitBoard) -> u32 {
    piece.bits.count_ones()
}

fn measure_control(state: &BoardState) -> f32 {
    // Form a movemap for all 64 positions and xor the current map.

    0.0
}

pub fn get_piece_coords(
    board: &BoardState,
    piece: PieceType,
    color: Color
) -> Vec<(u8, u8)> {
    let bitboard = board.get_bb(piece, color);

    let mut pieces = Vec::new();
    let mut bb = bitboard.bits;

    //=========================================================================
    // AI generated, verified.
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
    //=========================================================================

    pieces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_moves() {
    // Answer to rook on G2 moves.
    let r: u64 = 215893514783949314;

    let a: u64 = r;

    // Set b equal to rook on G2 bitboard.
    let mut b = 562949953421312;
        assert_eq!(a, b);
    }
}
