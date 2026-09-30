use crate::dw_engine::BitBoard;
// TODO: Convert u64 moveboards to BitBoard moveboards.

// For pawn, mut leave a "crumb" behind if moved 2 spaces on first move so
// that another pawn can au-passant to capture.

// Hopping piece movements.
pub const K_MOVS: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/k.txt"
);

pub const N_MOVS: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/n.txt"
);

pub const WP_MOVS: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/wp.txt"
);

pub const BP_MOVS: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/bp.txt"
);

pub const WP_CAPT: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/wpc.txt"
);

pub const BP_CAPT: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/bpc.txt"
);

// Blocker masks.
pub const R_MASK: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/r.txt"
);

pub const B_MASK: [u64; 64] = include!(
    "../../bitboard_generator/bitboards/b.txt"
);
