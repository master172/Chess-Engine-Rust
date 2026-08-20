use crate::{board::BoardState, input::InputPackage};

pub struct GameState {
    pub current_index: Option<i32>,
    pub legal_moves: u64,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_index: None,
            legal_moves: 0,
        }
    }

    pub fn input_to_game_state(&mut self, input: &mut InputPackage) {
        match input.left_mouse_index {
            Some(val) => self.current_index = Some(val),
            None => self.current_index = None,
        }
    }
}

pub fn handle_game_state(game_state: &mut GameState, board_state: &mut BoardState) {
    if game_state.current_index.is_none() {
        return;
    }
    if game_state.legal_moves != 0
        && (1 << game_state.current_index.unwrap()) & game_state.legal_moves != 0
    {
        println!("move piece over there");
    } else {
        board_state.generate_legal_moves(game_state);
    }
}
