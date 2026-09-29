use crate::dw_engine::BitBoard;

// Should be 64 magic rook numbers and 64 magic bishop numbers.

pub(crate) struct PieceMagic {
    move_boards: [Vec<BitBoard>; 64],
    blocker_boards: [Vec<BitBoard>; 64],
    blocker_masks: [BitBoard; 64], // For each square.
    magic_numbers: [BitBoard; 64],
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
    magic_number: BitBoard
) -> usize {
    ((blocker_board.bits as u128 * magic_number.bits as u128) >> 64) as usize
}

fn is_magic(candidate: BitBoard) -> bool {
    false
}
