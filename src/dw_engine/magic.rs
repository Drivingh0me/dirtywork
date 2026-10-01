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

fn build_blockers_and_moves(
    piece: SliderType,
    blocker_boards: &mut Vec<BitBoard>,
    move_boards: &mut Vec<BitBoard>,
) {

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
    println!("Finding Magic!");
}
