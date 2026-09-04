use crate::error::{Error, Result};

use crate::dw_engine::{BoardState};

pub fn print_board(board: BoardState) -> Result<()> {
    // Convert bitboard into Vec of peices.
    let w_pawn_coords = get_piece_coords(board.w_pawn.bit_board);

    dbg!(w_pawn_coords);
    Ok(())
}

fn get_piece_coords(bitboard: u64) -> Vec<(u8, u8)> {
    let mut pieces = Vec::new();
    let mut bb = bitboard;
    // AI generated, unverified.

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
    pieces
}

fn build_board() -> [char; 64] {
    let pieces: [char; 64] = [' '; 64];

    pieces
}

fn print_board_string(bp: [char; 64]) {
    for i in 0..7 {
        println!("{}{}{}{}{}{}{}{}",
            bp[i+0],
            bp[i+1],
            bp[i+2],
            bp[i+3],
            bp[i+4],
            bp[i+5],
            bp[i+6],
            bp[i+7]
        )
    }
}
