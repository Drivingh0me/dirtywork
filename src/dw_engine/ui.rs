use crate::error::{Error, Result};

use crate::dw_engine::{BoardState};

pub fn print_board(board: BoardState) -> Result<()> {
    // Convert bitboard into Vec of peices.
    let w_pawn_coords = get_piece_coords(board.w_pawn.bit_board);

    dbg!(&w_pawn_coords);

    let squares = build_board(w_pawn_coords);
    dbg!(squares);
    print_squares(squares);
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

fn build_board(pieces: Vec<(u8, u8)>) -> [char; 64] {
    let mut squares: [char; 64] = ['_'; 64];

    for p in pieces.iter() {
        squares[get_square_index(p)] = 'p';
    }

    squares
}

fn get_square_index(s: &(u8, u8)) -> usize {
    let i: usize = usize::from(s.0) + (usize::from(s.1) * 8);
    i
}

fn print_squares(sq: [char; 64]) {
    for i in 0..7 {
        println!("{}{}{}{}{}{}{}{}",
            sq[i * 8 + 0],
            sq[i * 8 + 1],
            sq[i * 8 + 2],
            sq[i * 8 + 3],
            sq[i * 8 + 4],
            sq[i * 8 + 5],
            sq[i * 8 + 6],
            sq[i * 8 + 7]
        )
    }
}
