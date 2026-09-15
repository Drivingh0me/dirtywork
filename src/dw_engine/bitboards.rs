use crate::dw_engine::BitBoard;

// King movements
pub const K_MOVS: [u64] = [include!(
    "../../bitboard_generator/bitboards/k.txt"
)];

pub const WP_MOVS: [u64] = [include!(
    "../../bitboard_generator/bitboards/wp.txt"
)];

pub const BP_MOVS: [u64] = [include!(
    "../../bitboard_generator/bitboards/bp.txt"
)];
