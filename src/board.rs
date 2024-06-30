// Board related stuff

use crate::bits;
use crate::defs::{Bitboard, Constants, EMPTY, Piece, Pieces, Side, Sides, Square, BB_SQUARES};
use crate::state::GameState;

mod fen;

#[derive(Clone)]
pub struct Board {
    pub pieces: [[Bitboard; Constants::PIECE_TYPES]; Sides::BOTH],
    pub side: [Bitboard; Sides::BOTH],
    pub game_state: GameState,
    pub piece_list: [Piece; Constants::SQUARES],
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: [[EMPTY; Constants::PIECE_TYPES]; Sides::BOTH],
            side: [EMPTY; Sides::BOTH],
            game_state: GameState::new(),
            piece_list: [Pieces::NONE; Constants::SQUARES],
        }
    }

    pub fn remove_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.pieces[side][piece] ^= BB_SQUARES[square];
        self.side[side] ^= BB_SQUARES[square];
        self.piece_list[square] = Pieces::NONE;
    }

    pub fn put_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.pieces[side][piece] |= BB_SQUARES[square];
        self.side[side] |= BB_SQUARES[square];
        self.piece_list[side] = piece;
    }

    pub fn move_piece(&mut self, side: Side, piece: Piece, from: Square, to: Square) {
        self.remove_piece(side, piece, from);
        self.put_piece(side, piece, to);
    }

    pub fn set_en_passant(&mut self, square: Square) {
        self.game_state.en_passant = Some(square as u8);
    }

    pub fn clear_en_passant(&mut self) {
        self.game_state.en_passant = None;
    }

    pub fn swap_side(&mut self) {
        self.game_state.active_color ^= 1;
    }

    pub fn update_castling_permissions(&mut self, new_permissions: u8) {
        self.game_state.castling_rights = new_permissions;
    }
}

impl Board {
    fn reset(&mut self) {
        self.pieces = [[0; Constants::PIECE_TYPES]; Sides::BOTH];
        self.side = [EMPTY; Sides::BOTH];
        self.game_state = GameState::new();
        self.piece_list = [Pieces::NONE; Constants::SQUARES];
    }

    fn init(&mut self) {
        let pieces_per_side = self.init_pieces_per_side();
        self.side[Sides::WHITE] = pieces_per_side.0;
        self.side[Sides::BLACK] = pieces_per_side.1;

        self.piece_list = self.init_piece_list();
    }

    // Init the pieces bitboard (used to keep track of overall side pieces)
    fn init_pieces_per_side(&self) -> (Bitboard, Bitboard) {
        let mut white: Bitboard = 0;
        let mut black: Bitboard = 0;

        for (w, b) in self.pieces[Sides::WHITE]
            .iter()
            .zip(self.pieces[Sides::BLACK].iter())
        {
            white |= *w;
            black |= *b;
        }

        (white, black)
    }

    fn init_piece_list(&self) -> [Piece; Constants::SQUARES] {
        let white = self.pieces[Sides::WHITE];
        let black = self.pieces[Sides::BLACK];
        let mut piece_list: [Piece; Constants::SQUARES] = [Pieces::NONE; Constants::SQUARES];

        for (piece_type, (w, b)) in white.iter().zip(black.iter()).enumerate() {
            let mut white_pieces = *w;
            let mut black_pieces = *b;

            while white_pieces > 0 {
                let square = bits::next(&mut white_pieces);
                piece_list[square] = piece_type;
            }

            while black_pieces > 0 {
                let square = bits::next(&mut black_pieces);
                piece_list[square] = piece_type;
            }
        }

        piece_list
    }
}
