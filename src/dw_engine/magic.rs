use std::collections::HashSet;

use crate::error::{Result, Error};
use crate::dw_engine::BitBoard;
use crate::dw_engine::bitboards;

// Notes ----------------------------------------------------------------------
// Magic is used to calculate the movebaord for slider. Moveboards for hoppers
// are precalculated completely and storred in bitboards.rs.
// Should be 64 magic rook numbers and 64 magic bishop numbers.
// ----------------------------------------------------------------------------

// TODO: Rename Slider::rook to Rook and Bishop to keep naming convention.

// Import magic numbers.
pub const K_MOVS: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/k.txt"
);

// Import magic numbers.
pub const R_MAGICS: [u64; 64] = include!(
    "../../magic/magic_rook.txt"
);

pub const B_MAGICS: [u64; 64] = include!(
    "../../magic/magic_bishop.txt"
);

enum Slider {
    bishop,
    rook,
}

pub(crate) struct PieceMagic {
    move_boards: [Vec<BitBoard>; 64],
    blocker_boards: [Vec<BitBoard>; 64],
    blocker_masks: [u64; 64], // For each square.
    magic_numbers: [u64; 64], // For each square.
    slider: Slider,
}

pub struct Magic {
    bishop: PieceMagic,
    rook: PieceMagic,
}

impl Magic {
    fn new() -> Self {
        Self {
            bishop: PieceMagic::new(Slider::bishop),
            rook: PieceMagic::new(Slider::rook),
        }
    }
}

impl PieceMagic {
    fn new(slider: Slider) -> Self {
        Self {
            move_boards: std::array::from_fn(|_| Vec::new()),
            blocker_boards: std::array::from_fn(|_| Vec::new()),
            blocker_masks: match slider {
                Slider::bishop => bitboards::B_MASK,
                Slider::rook => bitboards::R_MASK,
            },
            magic_numbers: match slider {
                Slider::bishop => B_MAGICS,
                Slider::rook => R_MAGICS,
            },
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
    let bb = blocker_board.bits;
    ((bb.wrapping_mul(magic_number)) >> (64 - blockers)) as usize
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
        Ok(())
    }

    fn see(&self, location: bitboards::Location) -> Result<SquareOccupancy> {
        let index = location.square()?;

        self.board.get(index).ok_or(Error::VectorSize).copied()
    }

    fn print(&self) {
        for row in (0..8).rev() {
            let mut a = Vec::new();
            for col in 0..8 {
                let location = bitboards::Location{ x: col, y: row };
                let occupancy = self.see(location).unwrap();
                let occ = match occupancy {
                    SquareOccupancy::Blocker => "1",
                    _ => "0",
                };
                a.push(occ);
            }
            println!("{}{}{}{}{}{}{}{}",
                a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]);
        }
    }

    fn step(&mut self,
        slide: impl Fn(bitboards::Location, usize) -> bitboards::Location,
        square: bitboards::Location,
        end: usize,
    ) -> Result<()> {
        for i in 1..end {
            let location = slide(square, i);
            match self.see(location)? {
                SquareOccupancy::Empty => {
                    self.set(
                        location.square()?,
                        SquareOccupancy::Myself
                    )?;
                },
                SquareOccupancy::Blocker => {
                    self.set(
                        location.square()?,
                        SquareOccupancy::Myself
                    )?;
                    break;
                },
                SquareOccupancy::Myself => {
                    return Err(Error::Collision(
                        String::from(
                            "While searching for step")
                    ));
                },
            }
        }
        Ok(())
    }

    fn moves(&mut self, piece: Slider, square: usize) -> Result<()> {
        let piece_location = bitboards::location(square);
        // Determine loop sizes for cardinal directions.
        let north = 8-piece_location.y;
        let south = piece_location.y;
        let east = 8-piece_location.x;
        let west = piece_location.x;

        let nw = std::cmp::min(north, west);
        let ne = std::cmp::min(north, east);
        let sw = std::cmp::min(south, west);
        let se = std::cmp::min(south, east);

        // Scan directions using movement algorithm for correct piece.
        match piece {
            Slider::rook => {
                // North.
                self.step(|mut loc, i| {
                        loc.y += i;
                        loc
                    },
                    piece_location,
                    north)?;

                // South.
                self.step(|mut loc, i| {
                        loc.y -= i;
                        loc
                    },
                    piece_location,
                    south)?;

                // East.
                self.step(|mut loc, i| {
                        loc.x += i;
                        loc
                    },
                    piece_location,
                    east)?;

                // West.
                self.step(|mut loc, i| {
                        loc.x -= i;
                        loc
                    },
                    piece_location,
                    west)?;
            },
            Slider::bishop => {
                // NorthWest.
                self.step(|mut loc, i| {
                        loc.y += i;
                        loc.x -= i;
                        loc
                    },
                    piece_location,
                    nw)?;

                // NorthEast.
                self.step(|mut loc, i| {
                        loc.y += i;
                        loc.x += i;
                        loc
                    },
                    piece_location,
                    ne)?;

                // SouthWest.
                self.step(|mut loc, i| {
                        loc.y -= i;
                        loc.x -= i;
                        loc
                    },
                    piece_location,
                    sw)?;

                // SouthEast.
                self.step(|mut loc, i| {
                        loc.y -= i;
                        loc.x += i;
                        loc
                    },
                    piece_location,
                    se)?;
            },
        }

        Ok(())
    }

    fn to_moveboard(&self) -> BitBoard {
        // Convert all Myself to 1 and else to 0.
        let mut out = BitBoard::new();
        for square in 0..64 {
            match self
                .see(bitboards::location(square))
                .unwrap_or(SquareOccupancy::Empty)
            {
                SquareOccupancy::Myself => out.bits |= 1u64 << square,
                _ => (),
            }
        }

        out
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
    let mut next_mask = blocker_mask;

    loop {
        out.push(next_mask);

        if next_mask.bits == 0 {
            break;
        }

        next_mask.bits = (next_mask.bits -1) & blocker_mask.bits;
    }

    out
}

fn build_moveboard(
    piece: Slider,
    blocker_board: BitBoard,
    square: usize
) -> Result<BitBoard> {
    // Convert blockerboard to MtxBoard.
    let mut board = blockerboard_to_mtxboard(blocker_board)?;

    // Find all squares the piece can move to or capture on.
    board.moves(piece, square)?;

    // Convert that MtxBoard to BitBoard.
    Ok(board.to_moveboard())
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

    let num_of_blockers: u32 = blocker_mask.count_ones();

    let table_size = 1usize << num_of_blockers;

    // cast_spell(blockers) is index in magic_movebards of moveboard.
    let mut magic_moveboards = Vec::new();

    let mut max_index = 0;

    'search: loop {
        magic_moveboards = vec![BitBoard::new(); table_size];

        candidate = rand::random::<u64>()
            & rand::random::<u64>()
            & rand::random::<u64>();

        let mut used = vec![false; table_size];
        for i in 0..blocker_boards.len() {
            let blockers: BitBoard = *blocker_boards
                .get(i)
                .ok_or(Error::ItemNotFound(String::from(
                    "blockerboard in blockerboards")))?;
            let moveboard: BitBoard = *move_boards
                .get(i)
                .ok_or(Error::ItemNotFound(String::from(
                    "moveboard in moveboards")))?;
            let index = cast_spell(blockers, candidate, num_of_blockers);

            if used[index] {
                if magic_moveboards[index].bits != moveboard.bits {
                    continue 'search;
                }
            } else {
                match used.get_mut(index) {
                    Some(elem) => *elem = true,
                    None => return Err(Error::FailedMutate(String::from(
                        "used magic indexes at index"))),
                }
                match magic_moveboards.get_mut(index) {
                    Some(elem) => *elem = moveboard,
                    None => return Err(Error::FailedMutate(String::from(
                        "magic_bitboards at index"))),
                }
            }

            if index > max_index {
                max_index = index;
            }
        }
        break;
    }
    Ok((candidate, magic_moveboards))
}

// Searches for magic numbers and records them in a file.
pub fn search_magic(magic: Magic) {
    println!("Finding Magic...");

    let mut rook_magic_numbers = Vec::new();
    let mut bishop_magic_numbers = Vec::new();

    // Find a magic number for each square for rook.
    for square in 0.. 64 {
        // Generate bitboards for this square.
        let mask = BitBoard::set_val(magic.rook.blocker_masks[square]);
        let blockers = build_blockers(mask);
        let mut moves = Vec::new();

        for blocker_set in &blockers {
            // println!("Searching next blockers");
            let moveboard = build_moveboard(
                Slider::rook,
                *blocker_set,
                square
            ).unwrap();
            moves.push(moveboard);
        }

        let (magic_number, magic_moves) = find_magic(
            &blockers,
            &moves,
            mask.bits
        ).unwrap();

        rook_magic_numbers.push(magic_number);
    }

    // Find a magic number for each sqaure for bishop.
    for square in 0.. 64 {
        // Generate bitboards for this square.
        let mask = BitBoard::set_val(magic.bishop.blocker_masks[square]);
        let blockers = build_blockers(mask);
        let mut moves = Vec::new();

        for blocker_set in &blockers {
            // println!("Searching next blockers");
            let moveboard = build_moveboard(
                Slider::bishop,
                *blocker_set,
                square
            ).unwrap();
            moves.push(moveboard);
        }

        let (magic_number, magic_moves) = find_magic(
            &blockers,
            &moves,
            mask.bits
        ).unwrap();

        bishop_magic_numbers.push(magic_number);
    }

    // Convert magics to an rust-style array that can be included.
    let rook_magic_string = format!("{:?}", rook_magic_numbers);
    let bishop_magic_string = format!("{:?}", bishop_magic_numbers);

    // Write magic/magic.txt, were normal magics are stored.
    std::fs::write("magic/magic_rook.txt", rook_magic_string).unwrap();
    std::fs::write("magic/magic_bishop.txt", bishop_magic_string).unwrap();

    // best magic/best_magic.txt is where some magics of fewer bits are stored.

    // TODO: Add fancy magics array.

    println!("Done!");
}
