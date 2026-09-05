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
    let pieces = PrintableBoard::getfrom(board);
    let squares = build_board(pieces);
    print_squares(squares);
    Ok(())
}

fn get_piece_coords(bitboard: u64) -> Vec<(u8, u8)> {
    let mut pieces = Vec::new();
    let mut bb = bitboard;

    // AI generated, human verified--------.
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
    //--------------------------------------.

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
        squares[get_square_index(p)] = '1';
    }

    for p in pieces.b_knight.iter() {
        squares[get_square_index(p)] = '2';
    }

    for p in pieces.b_bishop.iter() {
        squares[get_square_index(p)] = '3';
    }

    for p in pieces.b_rook.iter() {
        squares[get_square_index(p)] = '4';
    }

    for p in pieces.b_queen.iter() {
        squares[get_square_index(p)] = '5';
    }

    for p in pieces.b_king.iter() {
        squares[get_square_index(p)] = '6';
    }

    squares
}

fn get_square_index(s: &(u8, u8)) -> usize {
    let i: usize = usize::from(s.0) + (usize::from(s.1) * 8);
    i
}

fn print_squares(squares: [char; 64]) {
    let mut sq = squares.clone();
    sq.reverse();
    for i in 0..4 {
        println!("{}{}{}{}{}{}{}{}{}",
            9 - (2 * i + 1),
            ws(sq[2 * i * 8 + 0]),
            bs(sq[2 * i * 8 + 1]),
            ws(sq[2 * i * 8 + 2]),
            bs(sq[2 * i * 8 + 3]),
            ws(sq[2 * i * 8 + 4]),
            bs(sq[2 * i * 8 + 5]),
            ws(sq[2 * i * 8 + 6]),
            bs(sq[2 * i * 8 + 7])
        );
        println!("{}{}{}{}{}{}{}{}{}",
            9 - (2 * i + 2),
            bs(sq[(2 * i + 1) * 8 + 0]),
            ws(sq[(2 * i + 1) * 8 + 1]),
            bs(sq[(2 * i + 1) * 8 + 2]),
            ws(sq[(2 * i + 1) * 8 + 3]),
            bs(sq[(2 * i + 1) * 8 + 4]),
            ws(sq[(2 * i + 1) * 8 + 5]),
            bs(sq[(2 * i + 1) * 8 + 6]),
            ws(sq[(2 * i + 1) * 8 + 7])
        );
    }
    println!(" ABCDEFGH");
}

fn ws(p: char) -> String {
    match p {
        'p' => "\x1B[37;45mp\x1B[0m".to_string(),
        'n' => "\x1B[37;45mn\x1B[0m".to_string(),
        'b' => "\x1B[37;45mb\x1B[0m".to_string(),
        'r' => "\x1B[37;45mr\x1B[0m".to_string(),
        'q' => "\x1B[37;45mq\x1B[0m".to_string(),
        'k' => "\x1B[37;45mk\x1B[0m".to_string(),
        '1' => "\x1B[30;45mp\x1B[0m".to_string(),
        '2' => "\x1B[30;45mn\x1B[0m".to_string(),
        '3' => "\x1B[30;45mb\x1B[0m".to_string(),
        '4' => "\x1B[30;45mr\x1B[0m".to_string(),
        '5' => "\x1B[30;45mq\x1B[0m".to_string(),
        '6' => "\x1B[30;45mk\x1B[0m".to_string(),
        '_' => "\x1B[30;45m \x1B[0m".to_string(),
        _ => "x".to_string(),
    }
}

fn bs(p: char) -> String {
    match p {
        'p' => "\x1B[37;44mp\x1B[0m".to_string(),
        'n' => "\x1B[37;44mn\x1B[0m".to_string(),
        'b' => "\x1B[37;44mb\x1B[0m".to_string(),
        'r' => "\x1B[37;44mr\x1B[0m".to_string(),
        'q' => "\x1B[37;44mq\x1B[0m".to_string(),
        'k' => "\x1B[37;44mk\x1B[0m".to_string(),
        '1' => "\x1B[30;44mp\x1B[0m".to_string(),
        '2' => "\x1B[30;44mn\x1B[0m".to_string(),
        '3' => "\x1B[30;44mb\x1B[0m".to_string(),
        '4' => "\x1B[30;44mr\x1B[0m".to_string(),
        '5' => "\x1B[30;44mq\x1B[0m".to_string(),
        '6' => "\x1B[30;44mk\x1B[0m".to_string(),
        '_' => "\x1B[30;44m \x1B[0m".to_string(),
        _ => "x".to_string(),
    }
}
