// This is the file for all of the stuff related to pices, like determining
// moves and stuff.

use crate::engine::boards::*;
use crate::engine::magic::{self, Magic, get_slider_moveboard, Slider};

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
    // last 4 bits are flags: (these are all mutually exclusive)
    // Normal move,
    // Promotion to queen,
    // Promotion to rook,
    // Promotion to bishop,
    // Promotion to knight,
    // En passant,
    // Double pawn push,
    // White short castle,
    // White long castle,
    // Black short castle,
    // Black long castle,
    // Not a move / Error,
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

        // Debug
        // let debug_str =
        //     format!(": {} -> {}", start_square.index(), end_square.index());
        // out.push_str(&debug_str);

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
    let my_color = match color {
        Color::White => Occupier::White,
        Color::Black => Occupier::Black,
    };

    let friends = state.occupancy(my_color);

    let other_color = match color {
        Color::White => Occupier::Black,
        Color::Black => Occupier::White,
    };

    let foes = state.occupancy(other_color);

    let both_colors = Occupier::Both;

    let everyone = state.occupancy(both_colors);

    match piece {
        // This needs to be more complicated because of all of the pawn
        // capture mechanics and en passant.
        // TODO: Add en passant capture.
        PieceType::Pawn => match color {
            Color::White => {
                // Normal waling moovs.
                let mut walks = BitBoard::set_val(WP_MOVS[square.index()]);
                walks.remove(everyone);

                // Makes sure pawn respects blockers.
                if walks.how_many() == 1 {
                    let mut jumped = BitBoard::set_val(
                        WP_MOVS[square.index()]
                    );
                    jumped.clear_lowest();
                    walks.remove(jumped);
                }

                let mut captures = BitBoard::set_val(WP_CAPT[square.index()]);
                captures.remove(friends);

                captures.and(foes);

                walks.combine(captures);
                walks
            },
            Color::Black => {
                let mut walks = BitBoard::set_val(BP_MOVS[square.index()]);
                walks.remove(everyone);

                if walks.how_many() == 1 {
                    let mut jumped = BitBoard::set_val(
                        BP_MOVS[square.index()]
                    );
                    jumped.clear_lowest();
                    walks.and(jumped);
                }

                let mut captures = BitBoard::set_val(BP_CAPT[square.index()]);
                captures.remove(friends);

                let mut crumb: BitBoard = state.crumb();
                crumb.combine(foes);
                captures.and(crumb);

                walks.combine(captures);
                walks
            },
        },
        PieceType::Knight => {
            let mut moveboard = BitBoard::set_val(N_MOVS[square.index()]);
            moveboard.remove(friends);
            moveboard
        },
        PieceType::Bishop => {
            let mask = BitBoard::set_val(B_MASK[square.index()]);
            let mut blockers = mask;
            blockers.and(everyone);
            let max_blockers = mask.how_many();

            let mut moveboard = get_slider_moveboard(
                Slider::Bishop,
                square,
                blockers,
                max_blockers,
                magic
            );

            moveboard.remove(friends);
            moveboard
        },
        PieceType::Rook => {
            let mask = BitBoard::set_val(R_MASK[square.index()]);
            let mut blockers = mask;
            blockers.and(everyone);
            let max_blockers = mask.how_many();

            let mut moveboard = get_slider_moveboard(
                Slider::Rook,
                square,
                blockers,
                max_blockers,
                magic
            );

            moveboard.remove(friends);
            moveboard
        },
        // Just bitwiseAnd rook and bishop moveboards for queen.
        PieceType::Queen => {
            let mask = BitBoard::set_val(B_MASK[square.index()]);
            let mut blockers = mask;
            blockers.and(everyone);
            let max_blockers = mask.how_many();

            let mut moveboard = get_slider_moveboard(
                Slider::Bishop,
                square,
                blockers,
                max_blockers,
                magic
            );

            moveboard.remove(friends);

            let mask_rook = BitBoard::set_val(R_MASK[square.index()]);
            let mut blockers_rook = mask_rook;
            blockers_rook.and(everyone);
            let max_blockers_rook = mask_rook.how_many();

            let mut moveboard_rook = get_slider_moveboard(
                Slider::Rook,
                square,
                blockers_rook,
                max_blockers_rook,
                magic
            );

            moveboard_rook.remove(friends);

            moveboard.combine(moveboard_rook);
            moveboard

        },
        // Bitwise And with not(same color occupancy) to remove taking
        // own piece.
        // TODO: Remove moovs where king walks on square that can be captured.
        // TODO: Add castling with canceling if walks through check.
        PieceType::King => {
        let mut moveboard = BitBoard::set_val(K_MOVS[square.index()]);

        moveboard.remove(friends);
        moveboard
        },
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

    // TODO: Remove illegal moovs like not responding to check or ignoring
    // a pin against king.
    out
}

pub(crate) fn aggression(state: &BoardState) -> f32 {
    // See how close the average piece is to the king.
    let white_king = location(*state
        .squares(PieceType::King, Color::White)
        .get(0)
        .expect("Should only be one white king"));

    let black_king = location(*state
        .squares(PieceType::King, Color::Black)
        .get(0)
        .expect("Should only be one black king"));

    let mut white: f32 = 0.0;
    for piece in PieceType::ALL {
        let squares = state.squares(piece, Color::White);
        let mut i: usize = 0;
        loop {
            if squares[i] == Square::none() {
                break;
            }
            let location = location(squares[i]);
            let distance = location.distance(&black_king);
            white += 1.0 / distance;
            i += 1;
        }
    }

    let mut black: f32 = 0.0;
    for piece in PieceType::ALL {
        let squares = state.squares(piece, Color::Black);
        let mut i: usize = 0;
        loop {
            if squares[i] == Square::none() {
                break;
            }
            let location = location(squares[i]);
            let distance = location.distance(&white_king);
            black += 1.0 / distance;
            i += 1;
        }
    }

    white - black
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moovs() {
        assert_eq!(1, 1);
    }
}
