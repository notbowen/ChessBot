use crate::defs::{Piece, Pieces, Square, SQUARE_NAME};
use if_chain::if_chain;

pub type PotentialMove = (Square, Square, Piece);
pub type ParseMoveResult = Result<PotentialMove, ()>;

pub fn algebraic_square_to_number(algebraic_square: &str) -> Option<Square> {
    SQUARE_NAME
        .iter()
        .position(|&element| element == algebraic_square)
}
