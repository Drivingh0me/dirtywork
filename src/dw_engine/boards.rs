use crate::error::{Result, Error};

use super::pieces::{PieceType, Rights, Color, Occupier};

// TODO: Convert u64 moveboards to BitBoard moveboards.

// For pawn, mut leave a "crumb" behind if moved 2 spaces on first move so
// that another pawn can au-passant to capture.

// Bitboard is a1 -> h1 -> a2 -> h2... -> h8
#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct BitBoard {
    pub bits: u64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

    /// Calculates how many pieces there are.
    pub fn how_many(self) -> u32 {
        self.bits.count_ones()
    }

    // pub fn start(piece: PieceType, color: Color) -> Self {
    //     Self { bits: }
    // }
}

impl Square {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn none() -> Self {
        Self { bits: 65 }
    }

    pub(crate) fn set(val: u8) -> Self {
        Self { bits: val }
    }

    /// Check if square is valid.
    pub(crate) fn check(&self) -> bool {
        if self.bits > 63 {
            return false;
        }

        true
    }

    pub(crate) fn index(&self) -> usize {
        self.bits as usize
    }

    pub(crate) fn err() -> Self {
        Self { bits: 64 }
    }

    pub(crate) fn bits(&self) -> u16 {
        if self.bits > 63 {
            panic!("Tried to get square bits while err");
            return 0;
        }
        self.bits as u16
    }
}

// impl Div for Square {
//
// }

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
    rights: Rights,
    occupancies: [BitBoard; 3],
}

impl BoardState {
    fn new() -> Self {
        Self {
            pieces: [[BitBoard::new(); 6]; 2],
            crumb: Square::none(),
            rights: Rights::new(),
            occupancies: [BitBoard::new(); 3],
        }
    }

    pub(crate) fn squares(&self,
        piece: PieceType,
        color: Color
    ) -> [Square; 10] {
        let mut squares = [Square::none(); 10];
        let mut index = 0;
        let mut bb = self.pieces[color as usize][piece as usize].bits;

        // 10 is the max number of pieces of a type possible, Squares is
        // Square::none terminated or last is 10.
        while bb != 0 && index < 10 {
            // Get the index of the lowest set bit (0-63).
            let square = bb.trailing_zeros() as u8;

            squares[index] = Square::set(square);
            index += 1;

            // Clear the lowest set bit.
            bb &= bb - 1;
            // Find then return all of the squares that piecetype of that
            // color are on.
        }
        squares
    }

    /// Get the bitboard of a piece in the BoardState.
    pub(crate) fn get_bb(
        &self,
        piece: PieceType,
        color: Color
    ) -> BitBoard {
        self.pieces[color as usize][piece as usize]
    }

    pub(crate) fn occupancy(&self, occ: Occupier) -> BitBoard {
        self.occupancies[occ as usize]
    }

    pub(crate) fn make_moov(&self, moov: Moov) -> Self {
        let mut out = self;
        // Find Color and Piecetype.
        for bb in out.pieces {
            //
        }

        out
    }
}

impl Default for BoardState {
    /// Sets starting position for all pieces.
    fn default() -> Self {
        Self{
            pieces: [[
                BitBoard::set_val(65280),
                BitBoard::set_val(66),
                BitBoard::set_val(36),
                BitBoard::set_val(129),
                BitBoard::set_val(16),
                BitBoard::set_val(8),
            ], [
                BitBoard::set_val(71776119061217280),
                BitBoard::set_val(4755801206503243776),
                BitBoard::set_val(2594073385365405696),
                BitBoard::set_val(9295429630892703744),
                BitBoard::set_val(1152921504606846976),
                BitBoard::set_val(576460752303423488),
            ]],
            crumb: Square::none(),
            rights: Rights::new(),
            occupancies: [BitBoard::new(); 3],

            // tomove: Color::White,
            // w_pawn: BitBoard { bits: 65280 },
            // w_knight: BitBoard { bits: 66 },
            // w_bishop: BitBoard { bits: 36 },
            // w_rook: BitBoard { bits: 129 },
            // w_queen: BitBoard { bits: 16 },
            // w_king: BitBoard { bits: 8 },
            // b_pawn: BitBoard { bits: 71776119061217280 },
            // b_knight: BitBoard { bits: 4755801206503243776 },
            // b_bishop: BitBoard { bits: 2594073385365405696 },
            // b_rook: BitBoard { bits: 9295429630892703744 },
            // b_queen: BitBoard { bits: 1152921504606846976 },
            // b_king: BitBoard { bits: 576460752303423488 },
            // dbl_pawn: BitBoard { bits: 0 },
            // castle: CastleBool::new(),
        }
    }
}


#[derive(Clone, Copy)]
pub(crate) struct Location {
    pub x: usize,
    pub y: usize,
}

pub(crate) struct LongNotation {
    pub string: String,
}

impl Location {
    pub(crate) fn square(self) -> Square {
        let out = Square::set((self.x + self.y * 8) as u8);
        if out.check() {
            return Square::err();
        }

        out
    }

    pub(crate) fn index(self) -> usize {
        self.x + self.y * 8
    }
}

pub(crate) fn location(square: Square) -> Location {
    let x = square.bits % 8;
    let y = square.bits / 8;
    let out: Location = Location { x: x as usize, y: y as usize };
    out
}

