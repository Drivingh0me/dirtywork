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

use std::cmp::Reverse;

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

// Need to impl Ord.
struct Eval {
    depth: i8, // How many moovs ahead have been used to get eval.
    eval: i16, // Evaluation in centipawns.
    mate: i8, // How many moves till forced mate, zero is mate
    // and 255 is no mate.
}

impl Eval {
    fn new(depth: u8) -> Self {
        Self { depth: depth, eval: 0, mate: 0 }
    }

    fn update(&mut self, val: Eval) {
        if self.depth < 255 {
            self.depth += 1;
        }
        self.eval = val.eval;
        self.mate = val.mate;
    }
}

enum Fork {
    Branches(Vec<Branch>),
    Leaf(BoardState),
}

// Need to be able to store data about braches for tree.
struct Branch {
    moovs: MoovList,
    ratings: Vec<Rating>,
    children: Fork,
}

impl Branch {
    fn new(leaf: &BoardState, eval: &mut Eval, magic: &Magic) -> Self {
        let moovs = pieces::legal_moovs(leaf, magic);
        let ratings = Vec::with_capacity(moovs.len());

        // Bubble up the top eval.
        eval.update(evaluate_pos(leaf, magic));
        let depth = eval.depth;

        for i in 0..moovs.len() {
            ratings.push(Rating::new(i, depth));
        }

        Self {
            moovs: moovs,
            ratings: ratings,
            // A new branch is always one away from a leaf.
            children: Fork::Leaf(leaf),
        }
    }

    fn extend(&mut self) -> Eval {
        match self
            .children
            .get(i)
            .expect("children should have i") {
            Fork::Branches(branches) => {
                // Extend top branches.
                let top_branches = ;
                for branch in top_branches {
                    self.ratings
                        .get_mut(branch)
                        .expect("branch should be in ratings")
                        .set(branch.extend());
                }
            },
            Fork::Leaf(leaf) => {
                // Create new branches and rank them.
                let mut child_branches: Vec<Branch> = Vec::
                    with_capacity(self.moovs.len());

                for branch_moov in self.moovs.len() {
                    child_branches.push(Branch::new(
                        leaf.from_moov(branch_moov),
                        // This may not be indexing fresh ratings correctly?
                        self.ratings
                            .get_mut(branch_moov)
                            .expect("leaf branch should be in ratings")
                            .rating,
                        magic
                    ));
                }
                self.children = Fork::Branches(child_branches);
            },
        }
        sort_ratings(self.ratings);

        // Return top eval
        self.ratings.get(0).expect("There should be at least one rating")
    }
}

// Consider impl Ord.
#[derive(Copy, Clone)]
struct Rating {
    rating: Eval,
    index: u8, // Index of moovlist that this rating describes.
}

impl Rating {
    fn new(index: usize, depth: u8) -> Self {
        Self {
            rating: Eval::new(depth),
            index: index,
        }
    }

    fn set(&mut self, val: Self) {
        *self = val;
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

    // Get initial rating for all moovs.
    for i in 0..fundamental_moovs.len() {
        // Need to use this as a state, and recursively call tree
        // traversal to deepen select branches.
        tree.push(Branch::new());
    }

    // Extend branches while memory lasts.
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

// TODO: Rank as best first for whos turn it is, not just white.
// Or choose last elem when black is turn to play.
fn sort_ratings(ratings: &mut Vec<Rating>) {
    ratings.sort_by_key(|r| Reverse(r.rating));
}

// Static evaluation.
// TODO: Add checkmate knowledge.
fn evaluate_pos(state: &BoardState, magic: &Magic) -> Eval {
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
