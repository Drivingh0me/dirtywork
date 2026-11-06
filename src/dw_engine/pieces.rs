// This is the file for all of the stuff related to pices, like determining
// moves and stuff.

use crate::dw_engine::boards::*;
use crate::dw_engine::magic::{self, Magic, get_slider_moveboard, Slider};

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Color {
    #[default]
    White = 0,
    Black = 1,
}

impl Color {
    pub fn other(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Occupier {
    White = 0,
    Black = 1,
    Both = 2,
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

    pub(crate) fn get(&self, index: usize) -> Moov {
        // TODO: Make this return something if index is beyond len.
        if index > 255 {
            panic!("cannot index MoovList beyond 255");
        }
        self.moovs[index]
    }

    pub(crate) fn len(&self) -> u8 {
        self.len
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

    pub(crate) fn start(&self) -> Square {
        let val = (self.bits & 0b0011_1111) as u8;

        Square::set(val)
    }

    pub(crate) fn end(&self) -> Square {
        let val = ((self.bits >> 6) & 0b0011_1111) as u8;

        Square::set(val)
    }

    pub(crate) fn set_start(&mut self, square: Square) {
        if !square.check() {
            panic!("tried to set start of moov with square err");
        }

        self.bits |= square.index() as u16;
    }

    pub(crate) fn set_end(&mut self, square: Square) {
        if !square.check() {
            panic!("tried to set end of moov with square err");
        }

        // Instead of raw six, make a CONST that is 6.
        self.bits |= (square.index() as u16) << 6;
    }

    pub(crate) fn string(&self) -> String {
        let mut out = String::new();

        let start_square = self.start();
        let end_square = self.end();

        let start = LongNotation::from_square(start_square);
        let end = LongNotation::from_square(end_square);

        out.push_str(&start.string);
        out.push_str(&end.string);

        out
    }
}

/// Get the moveboard.
fn get_moveboard(
    state: &BoardState,
    piece: PieceType,
    color: Color,
    square: Square,
    magic: &Magic
) -> BitBoard {
    let friends = match color {
        Color::White => Occupier::White,
        Color::Black => Occupier::Black,
    };

    let foes = match color {
        Color::White => Occupier::Black,
        Color::Black => Occupier::White,
    };

    let all = Occupier::Both;

    match piece {
        // This needs to be more complicated because of all of the pawn
        // capture mechanics and en passant.
        PieceType::Pawn => match color {
            Color::White => BitBoard::set_val(
                WP_MOVS[square.index()] & !state.occupancy(all).bits
            ),
            Color::Black => BitBoard::set_val(
                BP_MOVS[square.index()] & !state.occupancy(all).bits
            ),
        },
        PieceType::Knight => BitBoard::set_val(
            N_MOVS[square.index()] & !state.occupancy(friends).bits
        ),
        PieceType::Bishop => {
            let mut moveboard = get_slider_moveboard(
                Slider::Bishop,
                square,
                BitBoard::set_val(
                    state.occupancy(all).bits & B_MASK[square.index()]
                ),
                B_MASK.get(square.index())
                    .expect("B_MASK to be size of index")
                    .count_ones(),
                magic
            );

            BitBoard::set_val(moveboard.bits & !state.occupancy(friends).bits)
        },
        PieceType::Rook => {
            let mut moveboard = get_slider_moveboard(
                Slider::Rook,
                square,
                BitBoard::set_val(
                    state.occupancy(all).bits & R_MASK[square.index()]
                ),
                R_MASK.get(square.index())
                    .expect("R_MASK to be size of index")
                    .count_ones(),
                magic
            );

            BitBoard::set_val(moveboard.bits & !state.occupancy(friends).bits)
        },
        // Just bitwiseAnd rook and bishop moveboards for queen.
        PieceType::Queen => BitBoard::new(),
        // Bitwise And with not(same color occupancy) to remove taking
        // own piece.
        PieceType::King => BitBoard::set_val(
            K_MOVS[square.index()] & !state.occupancy(friends).bits
        ),
    }
}

// Calculate legal moves.
pub(crate) fn legal_moovs(state: &BoardState, magic: &Magic) -> MoovList {
    let mut out = MoovList::new();

    // Determine what color is to move.
    let color = state.turn();

    for piece in PieceType::ALL.iter() {
        let squares_piecetype_occupies = state.squares(*piece, color);

        for square in squares_piecetype_occupies {
            // If not a square, end of array.
            if !square.check() {
                break;
            }

            // let from_square = square.bits();

            if square == Square::none() {
                break;
            }

            let moveboard = get_moveboard(
                state,
                *piece,
                color,
                square,
                magic
            );

            let mut index = 0;
            let mut bb = moveboard.bits;
            while bb != 0 {
                let mut moov = Moov::new();

                // Set start square for moov.
                moov.set_start(square);

                // Get the index of the lowest set bit (0-63).
                let to_square = bb.trailing_zeros() as u8;

                // Set to square for moov.
                moov.set_end(Square::set(to_square));

                // Clear the lowest set bit.
                bb &= bb - 1;

                out.push(moov);
            }
        }
    }

    // Remove the non-legal moves.
    out
}

