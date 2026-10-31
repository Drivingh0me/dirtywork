use crate::error::{Result, Error};

// TODO: Convert u64 moveboards to BitBoard moveboards.

// For pawn, mut leave a "crumb" behind if moved 2 spaces on first move so
// that another pawn can au-passant to capture.

// Bitboard is a1 -> h1 -> a2 -> h2... -> h8
#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct BitBoard {
    pub bits: u64,
}

pub struct Square {
    // 64 is invalid.
    bits: u8, // From a1 -> h1 -> b1 -> h8.
}

impl BitBoard {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn set_val(val: u64) -> Self {
        Self { bits: val }
    }
}

// Hopping piece movements.
pub const K_MOVS: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/k.txt"
);

pub const N_MOVS: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/n.txt"
);

pub const WP_MOVS: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/wp.txt"
);

pub const BP_MOVS: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/bp.txt"
);

pub const WP_CAPT: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/wpc.txt"
);

pub const BP_CAPT: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/bpc.txt"
);

// Blocker masks.
pub const R_MASK: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/r.txt"
);

pub const B_MASK: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/b.txt"
);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoardState {
    pieces: [[BitBoard; 6]; 2],
    crumb: Square, // Square where you can capture the invisible pawn.
    castle: Rights,
    occupancies: [BitBoard; 3],
}

impl BoardState {
    fn new() -> Self {
        Self {
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
            castle: Rights::new(),
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
            castle: CastleBool::new(),
        }
    }
}


#[derive(Clone, Copy)]
pub(crate) struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct LongNotation {
    pub string: String,
}

impl Location {
    pub(crate) fn square(self) -> Square {
        let out: usize = self.x + self.y *8;
        if out > 63 {
            return Square { bits: 64 };
        }

        out
    }
}

pub(crate) fn location(square: usize) -> Location {
    let x = square % 8;
    let y = square / 8;
    let out: Location = Location { x: x, y: y };
    out
}
