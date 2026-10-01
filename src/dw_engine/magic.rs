use std::collections::HashSet;

use crate::error::{Result, Error};
use crate::dw_engine::BitBoard;
use crate::dw_engine::bitboards;

// Notes ----------------------------------------------------------------------
// Magic is used to calculate the movebaord for slider. Moveboards for hoppers
// are precalculated completely and storred in bitboards.rs.
// Should be 64 magic rook numbers and 64 magic bishop numbers.
// ----------------------------------------------------------------------------

// Import blocker_masks from bitboard generator.

enum SliderType {
    bishop,
    rook,
}

pub(crate) struct PieceMagic {
    move_boards: [Vec<BitBoard>; 64],
    blocker_boards: [Vec<BitBoard>; 64],
    blocker_masks: [u64; 64], // For each square.
    magic_numbers: [u64; 64], // For each square.
    slider: SliderType,
}

pub struct Magic {
    bishop: PieceMagic,
    rook: PieceMagic,
}

impl Magic {
    fn new() -> Self {
        Self {
            bishop: PieceMagic::new(SliderType::bishop),
            rook: PieceMagic::new(SliderType::rook),
        }
    }
}

impl PieceMagic {
    fn new(slider: SliderType) -> Self {
        Self {
            move_boards: std::array::from_fn(|_| Vec::new()),
            blocker_boards: std::array::from_fn(|_| Vec::new()),
            blocker_masks: match slider {
                SliderType::bishop => bitboards::B_MASK,
                SliderType::rook => bitboards::R_MASK,
            },
            magic_numbers: [0u64; 64],
            slider: slider,
        }
    }
}

pub fn initialize_magic() -> Magic {
    let out = Magic::new();
    out
}

// Should return a unique index of moveboards for any blockerboard.
pub(crate) fn cast_spell(
    blocker_board: BitBoard,
    magic_number: u64,
    blockers: u32,
) -> usize {
    ((blocker_board.bits * magic_number) >> (64 - blockers)) as usize
}

#[derive(Copy, Clone)]
enum SquareOccupancy {
    Blocker,
    Myself,
    Empty,
}

pub(crate) struct MtxBoard {
    // board is always 64 elems and represents an 8x8 board.
    board: Vec<SquareOccupancy>,
}

impl MtxBoard {
    fn new() -> Self {
        Self {
            board: vec![SquareOccupancy::Empty; 64]
        }
    }

    fn set(&mut self, square: usize, occ: SquareOccupancy) -> Result<()> {
        match self.board.get_mut(square) {
            Some(occupancy) => *occupancy = occ,
            None => return Err(Error::VectorSize),
        }
        false
    }

    fn see(&self, location: bitboards::Location) -> Result<SquareOccupancy> {
        let index = location.x + location.y * 8;
        if index > 63 {
            return Err(Error:VectorSize);
        }
        self.board.get(index).ok_or(Error::VectorSize)?
    }

    fn moves(&mut self, piece: SliderType, square: usize) {
        let piece_location = bitboard::location(square);
        // Determine loop sizes for cardinal directions.
        let north = 8-piece_location.y;
        let south = piece_location.y;
        let east = 8-piece_location.x;
        let west = piece_location.x;

        // Scan directions using movement algo for correct piece.
        match piece {
            SliderType::rook => {
                for i in 0..north {
                    let location = piece_location.copy();
                    location.y = location.y + i;
                    match self.see(location)? {
                        SquareOccupancy::Empty => ,
                        SquareOccupancy::Blocker => ,
                        SquareOccupancy::Myself => ,
                    }
                }
            },
            SliderType::bishop => {
                a
            }
        }
    }

    fn to_bitboard(&self) -> Bitboard {
        a
    }
}

pub(crate) fn blockerboard_to_mtxboard(
    blockerboard: BitBoard
) -> Result<MtxBoard> {
    let mut out = MtxBoard::new();
    let mut bb = blockerboard.bits;

    while bb != 0 {
        let square = bb.trailing_zeros() as usize;
        out.set(square, SquareOccupancy::Blocker)?;
        bb &= bb - 1;
    }

    Ok(out)
}

// Builds all permutations of the blockers for a square.
fn build_blockers(blocker_mask: BitBoard) -> Vec<BitBoard> {
    let mut out = Vec::new();

    out
}

fn build_moveboard(
    piece: SliderType,
    blocker_board: BitBoard,
    square: usize
) -> BitBoard {
    // Convert blockerboard to MtxBoard.
    let mut board = blockerboard_to_mtxboard(blocker_board);

    // Find all squares the piece can move to or capture on.
    board.moves(piece, square);

    // Convert that MtxBoard to BitBoard.
    board.to_bitboard()
}

// Returns a magic number.
// May be best to make the indexed moveboard while doing this.
pub fn find_magic(
    blocker_boards: &Vec<BitBoard>,
    move_boards: &Vec<BitBoard>,
    blocker_mask: u64
) -> Result<(u64, Vec<BitBoard>)> {
    // Candidate magic number.
    let mut candidate:u64 = 0;

    let max_size = 10_000;

    // cast_spell(blockers) is index in magic_movebards of moveboard.
    let mut magic_moveboards: Vec<BitBoard> = Vec::new();

    'search: loop {
        candidate = rand::random::<u64>();
        candidate = candidate & rand::random::<u64>();
        candidate = candidate & rand::random::<u64>();
        let mut indexes = HashSet::new();
        for i in 0..blocker_boards.len() {
            let blockers: BitBoard = *blocker_boards
                .get(i)
                .ok_or(Error::VectorSize)?;
            let moveboard: BitBoard = *move_boards
                .get(i)
                .ok_or(Error::VectorSize)?;
            let num_of_blockers: u32 = blockers.bits.count_ones();
            let index = cast_spell(blockers, candidate, num_of_blockers);
            if index > max_size {
                continue 'search;
            }

            // Resize magic_moveboards if necessary.
            magic_moveboards.resize(index, BitBoard::new());

            if !indexes.insert(index) {
                if magic_moveboards.get(index)
                    .ok_or(Error::VectorSize)?.bits != moveboard.bits {
                    continue 'search;
                }
            } else {
                match magic_moveboards.get_mut(index) {
                    Some(elem) => *elem = moveboard,
                    None => return Err(Error::VectorSize),
                }
            }
        }
        break;
    }
    // Check len(magic_moveboards) to see how efficient candidate is.
    Ok((candidate, magic_moveboards))
}

// Searches for magic numbers and records them in a file.
pub fn search_magic() {
    println!("Finding Magic...");
    // Open magic/magic.txt, were best magics so far are stored.
    // best magic/best_magic.txt is where the lengths are stored.

    println!("Done!");
}
