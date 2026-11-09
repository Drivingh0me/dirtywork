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

// This  might not be necessary. Try only using BoardState.
pub struct GameState {
    haswon: Option<Color>,
    incheck: bool,
    board: Vec<BoardState>,
}

impl GameState {
    pub(crate) fn push(&mut self, board: BoardState) {
        self.board.push(board);
    }
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

struct Eval {
    eval: i16, // Evaluation in centipawns.
    mate: i8, // How many moves till forced mate, zero is no mate.
}

impl Eval {
    fn new() -> Self {
        Self { eval: 0, mate: 0 }
    }
}

// Need to be able to store data about braches for tree.
struct Branch {
    depth: u8,
    max: u16,
    min: u16,
}

#[derive(Copy, Clone)]
struct Rating {
    rating: u16,
    index: u8,
}

impl Rating {
    fn new() -> Self {
        Self {
            rating: 0,
            index: 0,
        }
    }
}

struct RatingList {
    ratings: [Rating; 256],
    len: u8,
}

impl RatingList {
    fn new() -> Self {
        Self {
            ratings: [Rating::new(); 256],
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len as usize
    }

    fn swap(&mut self, a: u8, b: u8) {
        if a > self.len || b > self.len {
            panic!("tried to swap an index of RatingList that does not exist");
        }

        let temp = self.ratings[a as usize];

        self.ratings[a as usize] = self.ratings[b as usize];

        self.ratings[b as usize] = temp;
    }

    // This needs to somehow split a ratingList.
    // Need to ve able to make a slice of this.
    fn split_at_mut(&mut self, index u8) {
        //
    }

    fn get (&self, i: u8) -> Rating {
        self.ratings[i as usize]
    }
}

// Need to make this continuously return according to UCI standard, and can
// stop. Make it return depth, score, etc  that can be parsed to
// form a UCI standard stdout through UI.
pub fn dw_analysis(
    state: &GameState,
    depth: usize,
    magic: &Magic,
) -> Result<(BoardState, f32)> {
    println!("thinking...");

    let board_state = state
        .board
        .last()
        .expect("There should always be a last boardstate in gamestate");

    let fundamental_moovs = pieces::legal_moovs(&board_state, &magic);

    let mut top_moov = Moov::new();
    let mut max = -100.0;

    // Use this to keep memory of branches and how search proceeds.
    // Maybe trim to top 64 most likely instead of full MoovList.
    let mut tree = Vec::<Branch>::new();

    for i in 0..fundamental_moovs.len() {
        // Need to use this as a state, and recursively call tree
        // traversal to deepen select branches.
        tree.push(Branch::new());
    }

    for i in 0..depth {
        //
    }

    println!("fundamentals are:");
    for i in 0..fundamental_moovs.len() {
        let moov = fundamental_moovs.get(i as usize);
        println!("{}", moov.string());
        let board = board_state.from_moov(moov);
        let eval = evaluate_pos(&board, &magic);

        if eval > max {
            top_moov = moov;
            max = eval;
        }
    }


    println!("top move is: {}", top_moov.string());

    let best_moov = board_state.from_moov(top_moov);

    println!("ran analysis!");
    Ok((best_moov, max))
}

// TODO: This needs a lot of work.
/// Sorts the moves acording to the ranking array.
fn rank_moovs(ratings: &mut RatingList) {
    // for i in 0..moovs.len() {
    //     let moov = moovs.get(i as usize);
    // }

    // A simple impl of quicksort.
    if ratings.len() <= 1 {
        return;
    }

    let pivot_index = partition(ratings);

    let (left, right) = ratings.split_at_mut(pivot_index);

    rank_moovs(left);
    rank_moovs(&mut right[1..]);
}

fn partition(ratings: &mut RatingList) -> usize {
    let len = ratings.len();

    // Update this to choose from first last or middle elem.
    // Set last element as pivot.
    let pivot_index = len - 1;

    let mut i = 0;
    for j in 0..pivot_index {
        // If the current element is less than or equal to the pivot.
        if ratings.get(j) <= ratings[pivot_index] {
            ratings.swap(i, j);
            i += 1;
        }
    }
    ratings.swap(i, pivot_index);
    i // pivot index.
}

// Static evaluation.
fn evaluate_pos(state: &BoardState, magic: &Magic) -> f32 {
    let material_weight: f32 = 0.5;
    let control_weight: f32 = 0.3;
    let agro_weight: f32 = 0.2;

    let material = count_material(&state);
    // println!("material is: {}", material);

    let control = measure_control(&state, &magic);
    // println!("control is: {}", control);

    let agro = aggression(&state);
    // println!("aggression is: {}", agro);

    (material_weight * material +
    control_weight * control +
    agro_weight * agro)
}

fn aggression(state: &BoardState) -> f32 {
    // See how close the average piece is to the king.

    0.0
}

fn count_material(state: &BoardState) -> f32 {
    let p = state.get_bb(PieceType::Pawn, Color::White).how_many()
        - state.get_bb(PieceType::Pawn, Color::Black).how_many();
    let n = state.get_bb(PieceType::Knight, Color::White).how_many()
        - state.get_bb(PieceType::Knight, Color::Black).how_many();
    let b = state.get_bb(PieceType::Bishop, Color::White).how_many()
        - state.get_bb(PieceType::Bishop, Color::Black).how_many();
    let r = state.get_bb(PieceType::Rook, Color::White).how_many()
        - state.get_bb(PieceType::Rook, Color::Black).how_many();
    let q = state.get_bb(PieceType::Queen, Color::White).how_many()
        - state.get_bb(PieceType::Queen, Color::Black).how_many();

    let p = p as f32;
    let n = n as f32;
    let b = b as f32;
    let r = r as f32;
    let q = q as f32;

    1.0 * p + 3.0 * n + 3.0 * b + 5.0 * r + 9.0 * q
}

fn measure_control(state: &BoardState, magic: &Magic) -> f32 {
    // Form a movemap for all 64 positions and xor the current map.
    let mut exp_state = *state;

    exp_state.set_turn(Color::White);

    let white_moovs = pieces::legal_moovs(
        &exp_state,
        &magic
    );

    let white_control = white_moovs.len() as f32;

    exp_state.set_turn(Color::White);

    let black_moovs = pieces::legal_moovs(
        &exp_state,
        &magic
    );

    let black_control = black_moovs.len() as f32;

    white_control / black_control
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
