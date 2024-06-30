#[derive(Clone)]
pub struct GameState {
    pub active_color: u8,
    pub castling_rights: u8,
    pub halfmove_clock: u8,
    pub en_passant: Option<u8>,
    pub fullmove_number: u16,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            active_color: 0,
            castling_rights: 0,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 0,
        }
    }
}
