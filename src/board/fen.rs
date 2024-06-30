use std::fmt;
use std::fmt::Display;
use std::ops::RangeInclusive;

use if_chain::if_chain;

use crate::defs::{BB_SQUARES, CastlingRights, FEN_START_POSITION, Files, MAX_GAME_MOVES, MAX_MOVE_RULE, Pieces, Ranks, Sides, Square, Squares};
use crate::misc::parse;

use super::Board;

const FEN_PARTS: usize = 6;
const LIST_OF_PIECES: &str = "kqrbnpKQRBNP";
const EP_SQUARES_WHITE: RangeInclusive<Square> = Squares::A3..=Squares::H3;
const EP_SQUARES_BLACK: RangeInclusive<Square> = Squares::A6..=Squares::H6;
const WHITE_OR_BLACK: &str = "wb";
const SPLITTER: char = '/';
const DASH: char = '-';
const EM_DASH: char = '–';
const SPACE: char = ' ';

pub enum FenError {
    IncorrectLength,
    PieceSquareError,
    ColorError,
    CastlingError,
    EnPassantError,
    HalfMoveClockError,
    FullMoveNumberError,
}

impl Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::IncorrectLength => "Error in FEN string: Must be 6 parts",
            Self::PieceSquareError => "Error in FEN string: Pieces or squares",
            Self::ColorError => "Error in FEN string: Colors",
            Self::CastlingError => "Error in FEN string: Castling rights",
            Self::EnPassantError => "Error in FEN string: En passant field",
            Self::HalfMoveClockError => "Error in FEN string: Half-move clock",
            Self::FullMoveNumberError => "Error in FEN string: Full-move number",
        };
        write!(f, "{error}")
    }
}

pub type FenResult = Result<(), FenError>;
pub type SplitResult = Result<Vec<String>, FenError>;
type FenPartParser = fn(board: &mut Board, part: &str) -> FenResult;

impl Board {
    pub fn fen_setup(&mut self, fen_string: Option<&str>) -> FenResult {
        let fen_parts = split_fen_string(fen_string)?;

        let fen_parsers = create_part_parsers();

        let mut temp_board = self.clone();
        temp_board.reset();

        for (parser, part) in fen_parsers.iter().zip(fen_parts.iter()) {
            parser(&mut temp_board, part)?;
        }

        temp_board.init();
        *self = temp_board;

        Ok(())
    }
}

fn split_fen_string(fen_string: Option<&str>) -> SplitResult {
    const SHORT_FEN_LENGTH: usize = 4;

    let mut fen_string: Vec<String> = fen_string
        .unwrap_or_else(|| FEN_START_POSITION)
        .replace(EM_DASH, DASH.encode_utf8(&mut [0; 4]))
        .split(SPACE)
        .map(String::from)
        .collect();

    if fen_string.len() == SHORT_FEN_LENGTH {
        fen_string.append(&mut vec![String::from("0"), String::from("1")]);
    }

    if fen_string.len() != FEN_PARTS {
        return Err(FenError::IncorrectLength);
    }

    Ok(fen_string)
}

fn create_part_parsers() -> [FenPartParser; FEN_PARTS] {
    [
        pieces,
        color,
        castling,
        en_passant,
        half_move_clock,
        full_move_number,
    ]
}

fn pieces(board: &mut Board, part: &str) -> FenResult {
    let mut rank = Ranks::R8 as u8;
    let mut file = Files::A as u8;

    for c in part.chars() {
        let square = ((rank * 8) + file) as usize;
        match c {
            'k' => board.pieces[Sides::BLACK][Pieces::KING] |= BB_SQUARES[square],
            'q' => board.pieces[Sides::BLACK][Pieces::QUEEN] |= BB_SQUARES[square],
            'r' => board.pieces[Sides::BLACK][Pieces::ROOK] |= BB_SQUARES[square],
            'b' => board.pieces[Sides::BLACK][Pieces::BISHOP] |= BB_SQUARES[square],
            'n' => board.pieces[Sides::BLACK][Pieces::KNIGHT] |= BB_SQUARES[square],
            'p' => board.pieces[Sides::BLACK][Pieces::PAWN] |= BB_SQUARES[square],
            'K' => board.pieces[Sides::WHITE][Pieces::KING] |= BB_SQUARES[square],
            'Q' => board.pieces[Sides::WHITE][Pieces::QUEEN] |= BB_SQUARES[square],
            'R' => board.pieces[Sides::WHITE][Pieces::ROOK] |= BB_SQUARES[square],
            'B' => board.pieces[Sides::WHITE][Pieces::BISHOP] |= BB_SQUARES[square],
            'N' => board.pieces[Sides::WHITE][Pieces::KNIGHT] |= BB_SQUARES[square],
            'P' => board.pieces[Sides::WHITE][Pieces::PAWN] |= BB_SQUARES[square],
            '1'..='8' => {
                if let Some(x) = c.to_digit(10) {
                    file += x as u8;
                }
            }
            SPLITTER => {
                if file != 8 {
                    return Err(FenError::PieceSquareError);
                }
                rank -= 1;
                file = 0;
            }
            _ => return Err(FenError::PieceSquareError),
        }

        if LIST_OF_PIECES.contains(c) {
            file += 1;
        }
    }

    Ok(())
}

fn color(board: &mut Board, part: &str) -> FenResult {
    if_chain! {
        if part.len() == 1;
        if let Some(c) = part.chars().next();
        if WHITE_OR_BLACK.contains(c);
        then {
            match c {
                'w' => board.game_state.active_color = Sides::WHITE as u8,
                'b' => board.game_state.active_color = Sides::BLACK as u8,
                _ => (),
            }
            return Ok(())
        }
    }

    Err(FenError::ColorError)
}

fn castling(board: &mut Board, part: &str) -> FenResult {
    if part.len() > 4 || part.len() < 1 {
        return Err(FenError::CastlingError);
    }

    for c in part.chars() {
        match c {
            'K' => board.game_state.castling_rights |= CastlingRights::WHITE_KING,
            'Q' => board.game_state.castling_rights |= CastlingRights::WHITE_QUEEN,
            'k' => board.game_state.castling_rights |= CastlingRights::BLACK_KING,
            'q' => board.game_state.castling_rights |= CastlingRights::BLACK_QUEEN,
            '-' => (),
            _ => return Err(FenError::CastlingError)
        }
    }

    Ok(())
}

fn en_passant(board: &mut Board, part: &str) -> FenResult {
    if_chain! {
        if part.len() == 1;
        if let Some(x) = part.chars().next();
        if x == DASH;
        then {
            return Ok(());
        }
    }

    if part.len() == 2 {
        let square = parse::algebraic_square_to_number(part);

        return match square {
            Some(sq) if EP_SQUARES_WHITE.contains(&sq) || EP_SQUARES_BLACK.contains(&sq) => {
                board.game_state.en_passant = Some(sq as u8);
                Ok(())
            }
            _ => Err(FenError::EnPassantError),
        };
    }

    Err(FenError::EnPassantError)
}

fn half_move_clock(board: &mut Board, part: &str) -> FenResult {
    if_chain! {
        if (1..=3).contains(&part.len());
        if let Ok(x) = part.parse::<u8>();
        if x <= MAX_MOVE_RULE;
        then {
            board.game_state.halfmove_clock = x;
            return Ok(())
        }
    }

    Err(FenError::HalfMoveClockError)
}

fn full_move_number(board: &mut Board, part: &str) -> FenResult {
    if_chain! {
        if !part.is_empty() && part.len() <= 4;
        if let Ok(x) = part.parse::<u16>();
        if x <= (MAX_GAME_MOVES as u16);
        then {
            board.game_state.fullmove_number = x;
            return Ok(());
        }
    }

    Err(FenError::FullMoveNumberError)
}
