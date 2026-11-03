// This is the file for all of the stuff related to pices, like determining
// moves and stuff.

use crate::dw_engine::boards::*;
use crate::dw_engine::magic::{self};

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Color {
    #[default]
    White = 0,
    Black = 1,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Occupier {
    White = 0,
    Black = 1,
    Both = 3,
}

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    const ALL: [Self; 6] = [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ];
}

// Make a movelist that is stack alocated and can hold up to 256 moves.
pub(crate) struct MoovList {
    moovs: [Moov; 256],
    len: u8, // Length of the array.
}

impl MoovList {
    pub(crate) fn new() -> Self {
        Self {
            moovs: [Moov::none(); 256],
            len: 0,
        }
    }

    pub(crate) fn push(&mut self, moov: Moov) {
        if self.len > 255 {
            // Make this not silent.
            return ();
        }
        self.moovs[self.len as usize] = moov;
        self.len += 1;
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Moov {
    // First 6 bits are from square, second 6 bits are to square,
    // last 4 bits are flags:
    // Normal move,
    // Promotion to queen,
    // Promotion to rook,
    // Promotion to bishop,
    // Promotion to knight,
    // En passant,
    // Double pawn push,
    // White short castle,
    // White ong castle,
    // Black short castle,
    // Black long castle,
    // Not a move,
    bits: u16,
}

impl Moov {
    pub(crate) fn new() -> Self {
        Self { bits: 0 }
    }
    pub(crate) fn none() -> Self {
        Self { bits: 0 }
    }
}

struct MoovPiece {
    moovs: [Moov; 27],
    len: u8,
}

impl MoovPiece {
    fn new() -> Self {
        Self {
            moovs: [Moov::new(); 27],
            len: 0,
        }
    }

    fn push(&mut self, moov: Moov) {
        if self.len > 26 {
            // Make this not silent.
            return ();
        }
        self.moovs[self.len as usize] = moov;
        self.len += 1;
    }
}

pub(crate) const WHITE_TO_MOVE: u8 = 0b1000_0000; // White's turn to move.
pub(crate) const BLACK_TO_MOVE: u8 = 0b0000_0000; // Black's turn to move.
pub(crate) const WHITE_OO: u8 = 0b0000_0001; // White can short castle
pub(crate) const WHITE_OOO: u8 = 0b0000_0010; // White can long castle
pub(crate) const BLACK_OO: u8 = 0b0000_0100; // Black can short castle
pub(crate) const BLACK_OOO: u8 = 0b0000_1000; // Black can long castle
pub(crate) const NO_CASTLE: u8 = 0b0000_0000;
pub(crate) const ANY_CASTLE: u8 = 0b0000_1111;
pub(crate) const DEFAULT_RIGHTS: u8 = 0b1000_1111;

// Stores if the rooks or kings have moved to stop casttle rights.
#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Rights {
    bits: u8,
}

impl Rights {
    pub(crate) fn new() -> Self {
        Self {
            bits: DEFAULT_RIGHTS,
        }
    }

    /// Grant a specific castling right.
    pub(crate) fn grant(&mut self, right: u8) {
        self.bits |= right;
    }

    /// Revoke a specific castling right.
    pub(crate) fn revoke(&mut self, right: u8) {
        self.bits &= !right;
    }

    /// Check if a specific castling right is available.
    pub(crate) fn has(&mut self, right: u8) -> bool {
        (self.bits & right) == right
    }
}

/// Get all of the moves one piece on one square can make.
fn get_moovs(
    state: &BoardState,
    piece: PieceType,
    color: Color,
    square: Square
) -> MoovPiece {
    let mut out = Moov::new();
    out
}

// Calculate legal moves.
pub(crate) fn legal_moovs(state: &BoardState) -> MoovList {
    let mut out = MoovList::new();

    // Determine what color is to move.
    let color = Color::White;

    // Get all moves for the jumping pieces.

    // Get all moves for the sliding pieces.
    for piece in PieceType::ALL.iter() {
        let squares_piecetype_occupies = state.squares(*piece, color);

        // Make this end when index is over or at max.
        for square in squares_piecetype_occupies {
            let moovs = get_moovs(
                state,
                *piece,
                color,
                square
            );
        }
    }

    // Remove the non-legal moves.
    out
}

