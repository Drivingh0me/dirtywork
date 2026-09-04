use crate::error::{Error, Result};

use crate::dw_engine::{BoardState};

#[derive(Debug)]
struct PrintableBoard {
    w_pawn: Vec<(u8, u8)>,
    w_knight: Vec<(u8, u8)>,
    w_bishop: Vec<(u8, u8)>,
    w_rook: Vec<(u8, u8)>,
    w_queen: Vec<(u8, u8)>,
    w_king: Vec<(u8, u8)>,
    b_pawn: Vec<(u8, u8)>,
    b_knight: Vec<(u8, u8)>,
    b_bishop: Vec<(u8, u8)>,
    b_rook: Vec<(u8, u8)>,
    b_queen: Vec<(u8, u8)>,
    b_king: Vec<(u8, u8)>,
}

impl PrintableBoard {
    fn getfrom (board: BoardState) -> Self {
        Self {
            w_pawn: get_piece_coords(board.w_pawn.bit_board),
            w_knight: get_piece_coords(board.w_knight.bit_board),
            w_bishop: get_piece_coords(board.w_bishop.bit_board),
            w_rook: get_piece_coords(board.w_rook.bit_board),
            w_queen: get_piece_coords(board.w_queen.bit_board),
            w_king: get_piece_coords(board.w_king.bit_board),
            b_pawn: get_piece_coords(board.b_pawn.bit_board),
            b_knight: get_piece_coords(board.b_knight.bit_board),
            b_bishop: get_piece_coords(board.b_bishop.bit_board),
            b_rook: get_piece_coords(board.b_rook.bit_board),
            b_queen: get_piece_coords(board.b_queen.bit_board),
            b_king: get_piece_coords(board.b_king.bit_board),
        }
    }
}

pub fn print_board(board: BoardState) -> Result<()> {
    // Convert bitboard into Vec of peices.
    // let w_pawn_coords = get_piece_coords(board.w_pawn.bit_board);
    let pieces = PrintableBoard::getfrom(board);

    dbg!(&pieces);

    let squares = build_board(pieces);
    dbg!(squares);
    print_squares(squares);
    Ok(())
}

fn get_piece_coords(bitboard: u64) -> Vec<(u8, u8)> {
    let mut pieces = Vec::new();
    let mut bb = bitboard;

    // AI generated, verified.
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

fn build_board(pieces: PrintableBoard) -> [char; 64] {
    let mut squares: [char; 64] = ['_'; 64];

    for p in pieces.w_pawn.iter() {
        squares[get_square_index(p)] = 'p';
    }

    for p in pieces.w_knight.iter() {
        squares[get_square_index(p)] = 'n';
    }

    for p in pieces.w_bishop.iter() {
        squares[get_square_index(p)] = 'b';
    }

    for p in pieces.w_rook.iter() {
        squares[get_square_index(p)] = 'r';
    }

    for p in pieces.w_queen.iter() {
        squares[get_square_index(p)] = 'q';
    }

    for p in pieces.w_king.iter() {
        squares[get_square_index(p)] = 'k';
    }

    for p in pieces.b_pawn.iter() {
        squares[get_square_index(p)] = 'p';
    }

    for p in pieces.b_knight.iter() {
        squares[get_square_index(p)] = 'n';
    }

    for p in pieces.b_bishop.iter() {
        squares[get_square_index(p)] = 'b';
    }

    for p in pieces.b_rook.iter() {
        squares[get_square_index(p)] = 'r';
    }
    
    for p in pieces.b_queen.iter() {
        squares[get_square_index(p)] = 'q';
    }

    for p in pieces.b_king.iter() {
        squares[get_square_index(p)] = 'k';
    }

    squares
}

fn build_boardx(pieces: Vec<(u8, u8)>) -> [char; 64] {
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
    for i in 0..8 {
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
