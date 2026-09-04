
use crate::error::{Error, Result};

use crate::dw_engine::{BoardState};

pub fn print_board(board: BoardState) -> Result<()> {
    // Convert bitboard into Vec of peices.
    // Assuming a 64-bit integer represents the bitboard
    let mut pieces = Vec::new();
    let mut bb:u64 = board.w_pawn.bit_board;

    while bb != 0 {
        // Get the index of the lowest set bit (0-63)
        let square = bb.trailing_zeros() as u8;

        // Convert index to coordinates (file and rank)
        let file = square % 8;
        let rank = square / 8;
        pieces.push((file, rank));
        // Clear the lowest set bit
        bb &= bb - 1;
    }

    dbg!(pieces);
    Ok(())
}
