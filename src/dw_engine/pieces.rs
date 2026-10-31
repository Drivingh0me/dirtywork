// This is the file for all of the stuff related to pices, like determining
// moves and stuff.

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Color {
    #[default]
    White = 0,
    Black = 1,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
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

// Make a movelist that is stack alocated and can hold up to 256 moves.

pub struct MovePiece {
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
    val: u16,
}

pub const WHITE_TO_MOVE: u8 = 0b1000_0000; // White's turn to move.
pub const BLACK_TO_MOVE: u8 = 0b0000_0000; // Black's turn to move.
pub const WHITE_OO: u8 = 0b0000_0001; // White can short castle
pub const WHITE_OOO: u8 = 0b0000_0010; // White can long castle
pub const BLACK_OO: u8 = 0b0000_0100; // Black can short castle
pub const BLACK_OOO: u8 = 0b0000_1000; // Black can long castle
pub const NO_CASTLE: u8 = 0b0000_0000;
pub const ANY_CASTLE: u8 = 0b0000_1111;
pub const DEFAULT_RIGHTS: u8 = 0b1000_1111;

// Stores if the rooks or kings have moved to stop casttle rights.
#[derive(Debug, PartialEq, Clone, Copy)]
struct Rights {
    val: u8,
}

impl Rights {
    fn new() -> Self {
        Self {
            val: DEFAULT_RIGHTS,
        }
    }

    /// Grant a specific castling right.
    fn grant(&mut self, right: u8) {
        self.val |= right;
    }

    /// Revoke a specific castling right.
    fn revoke(&mut self, right: u8) {
        self.val &= !right;
    }

    /// Check if a specific castling right is available.
    fn has(&mut self, right: u8) -> bool {
        (self.val & right) == right
    }
}

// Calculate legal moves.
fn legal_moves(state: &BoardState) -> Vec<MovePiece> {
    let mut out = Vec::<MovePiece>::new();

    // Get all moves for the jumping pieces.

    // Get all moves for the sliding pieces.
    let rook_moves = get_moveboards(
        Slider::Rook,
        blockers,
        &magic
    );

    // Remove the non-legal moves.
    out
}

