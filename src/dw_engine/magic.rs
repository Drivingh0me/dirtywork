use crate::dw_engine::BitBoard;
use std::collections::HashSet;

// Notes ----------------------------------------------------------------------
// Should be 64 magic rook numbers and 64 magic bishop numbers.
// ----------------------------------------------------------------------------

// Import blocker_masks from bitboard generator.

pub(crate) struct PieceMagic {
    move_boards: [Vec<BitBoard>; 64],
    blocker_boards: [Vec<BitBoard>; 64],
    blocker_masks: [u64; 64], // For each square.
    magic_numbers: [u64; 64], // For each square.
}

pub struct Magic {
    bishop: PieceMagic,
    rook: PieceMagic,
}

impl Magic {
    fn new() -> Self {
        Self {
            bishop: PieceMagic::new(),
            rook: PieceMagic::new(),
        }
    }
}

impl PieceMagic {
    fn new() -> Self {
        Self {
            move_boards: std::array::from_fn(|_| Vec::new()),
            blocker_boards: std::array::from_fn(|_| Vec::new()),
            blocker_masks: [BitBoard { bits: 0 }; 64],
            magic_numbers: [BitBoard { bits: 0 }; 64],
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
    magic_number: u64
) -> usize {
    ((blocker_board.bits as u128 * magic_number as u128) >> 64) as usize
}

// Returns a magic number.
// May be best to make the indexed moveboard while doing this.
fn find_magic(
    blocker_boards: &Vec<BitBoard>,
    move_boards: &Vec<BitBoard>
) -> BitBoard {
    let mut candidate = BitBoard::new();
    loop {
        candidate.bits = rand::random::<u64>();
        let mut indexes = HashSet::new();
        for blockers in blocker_boards {
            let index = cast_spell(*blockers, candidate);
            if !indexes.insert(index) {
                // i is index of this blockerboard and j is index of previous
                // blockerboard with same index from magic.
                if move_boards[i] != move_boards[j] {
                    continue;
                }
            }
        }
        break;
    }
    candidate
}
