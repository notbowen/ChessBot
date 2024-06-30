use crate::defs::{Bitboard, Square};

// Finds the next set bit in the bitboard and unsets it.
// Returns the location of the unset bit
pub fn next(bitboard: &mut Bitboard) -> Square {
    let square = bitboard.trailing_zeros() as Square;
    *bitboard ^= 1u64 << square;
    square
}