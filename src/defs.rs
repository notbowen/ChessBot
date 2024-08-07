// Definitions for the chess bot

use std::ops::RangeInclusive;

pub type Bitboard = u64;
pub type Side = usize;
pub type Piece = usize;
pub type Square = usize;

pub const EMPTY: u64 = 0;
pub const MAX_GAME_MOVES: usize = 2048;
pub const MAX_LEGAL_MOVES: u8 = 255;
pub const MAX_PLY: i8 = 125;
pub const MAX_MOVE_RULE: u8 = 100;

pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Sides;
impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const BOTH: Side = 2;
}

pub struct Constants;
impl Constants {
    pub const PIECE_TYPES: usize = 6;
    pub const CASTLING_PERMISSIONS: usize = 16;
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
}

pub struct CastlingRights;
impl CastlingRights {
    pub const WHITE_KING: u8 = 1;
    pub const WHITE_QUEEN: u8 = 2;
    pub const BLACK_KING: u8 = 4;
    pub const BLACK_QUEEN: u8 = 8;
    pub const ALL: u8 = 15;
}

#[rustfmt::skip]
pub const SQUARE_NAME: [&str; Constants::SQUARES] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"
];
pub const PIECE_NAME: [&str; Constants::PIECE_TYPES + 1] =
    ["King", "Queen", "Rook", "Bishop", "Knight", "Pawn", "-"];
pub const PIECE_CHAR_CAPS: [&str; Constants::PIECE_TYPES + 1] = ["K", "Q", "R", "B", "N", "", "_"];
pub const PIECE_CHAR_SMALL: [&str; Constants::PIECE_TYPES + 1] = ["k", "q", "r", "b", "n", "", ""];

pub struct Files;
impl Files {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const G: usize = 6;
    pub const H: usize = 7;
}

pub struct Ranks;
impl Ranks {
    pub const R1: usize = 0;
    pub const R2: usize = 1;
    pub const R4: usize = 3;
    pub const R5: usize = 4;
    pub const R7: usize = 6;
    pub const R8: usize = 7;
}

pub struct Squares;
impl Squares {
    pub const A1: Square = 0;
    pub const B1: Square = 1;
    pub const C1: Square = 2;
    pub const D1: Square = 3;
    pub const E1: Square = 4;
    pub const F1: Square = 5;
    pub const G1: Square = 6;
    pub const H1: Square = 7;

    pub const A8: Square = 56;
    pub const B8: Square = 57;
    pub const C8: Square = 58;
    pub const D8: Square = 59;
    pub const E8: Square = 60;
    pub const F8: Square = 61;
    pub const G8: Square = 62;
    pub const H8: Square = 63;

    pub const A3: Square = 16;
    pub const H3: Square = 23;

    pub const A6: Square = 40;
    pub const H6: Square = 47;
}

pub struct RangeOf;
impl RangeOf {
    pub const RANKS: RangeInclusive<u8> = (Ranks::R1 as u8)..=(Ranks::R8 as u8);
    pub const FILES: RangeInclusive<u8> = (Files::A as u8)..=(Files::H as u8);
    pub const SQUARES: RangeInclusive<Square> = 0..=63;
}

pub struct Pieces;
impl Pieces {
    pub const KING: Piece = 0;
    pub const QUEEN: Piece = 1;
    pub const ROOK: Piece = 2;
    pub const BISHOP: Piece = 3;
    pub const KNIGHT: Piece = 4;
    pub const PAWN: Piece = 5;
    pub const NONE: Piece = 6;
}

type BBFiles = [Bitboard; Constants::FILES];
type BBRanks = [Bitboard; Constants::RANKS];
type BBSquares = [Bitboard; Constants::SQUARES];

const fn init_files() -> BBFiles {
    const FILE_A: Bitboard = 0x0101_0101_0101_0101; // Represents the positions of file A
    let mut files: BBFiles = [0; Constants::FILES];
    let mut i = 0;

    while i < Constants::FILES {
        files[i] = FILE_A << i;
        i += 1;
    }

    files
}

const fn init_ranks() -> BBRanks {
    const RANK_1: Bitboard = 0xFF; // Represents the position of rank 1
    let mut ranks: BBRanks = [0; Constants::RANKS];
    let mut i = 0;

    while i < Constants::RANKS {
        ranks[i] = RANK_1 << (i * 8);
        i += 1;
    }

    ranks
}

const fn init_squares() -> BBSquares {
    let mut squares: BBSquares = [0; Constants::SQUARES];
    let mut i = 0;

    while i < Constants::SQUARES {
        squares[i] = 1u64 << i;
        i += 1;
    }

    squares
}

pub const BB_FILES: BBFiles = init_files();
pub const BB_RANKS: BBRanks = init_ranks();
pub const BB_SQUARES: BBSquares = init_squares();
