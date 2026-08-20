use crate::{
    board::{BoardState, pieces::Sides},
    input::InputPackage,
};

pub struct GameState {
    pub dev_mode: bool,
    pub current_side: Sides,

    pub previous_index: Option<i32>,
    pub current_index: Option<i32>,
    pub current_array_index: Option<usize>,
    pub legal_moves: u64,
}

pub enum MoveResult {
    Idle,
    Move,
    Generate,
}

impl GameState {
    pub fn new(dev_mode: bool, side: Sides) -> Self {
        Self {
            current_index: None,
            legal_moves: 0,
            previous_index: None,
            current_array_index: None,
            dev_mode,
            current_side: side,
        }
    }

    pub fn input_to_game_state(&mut self, input: &mut InputPackage) {
        match input.left_mouse_index {
            Some(val) => self.current_index = Some(val),
            None => self.current_index = None,
        }
    }
}

pub fn handle_game_state(game_state: &mut GameState, board_state: &mut BoardState) -> MoveResult {
    if game_state.current_index.is_none() {
        return MoveResult::Idle;
    }
    if game_state.legal_moves != 0
        && (1 << game_state.current_index.unwrap()) & game_state.legal_moves != 0
    {
        board_state.move_piece(game_state);
        return MoveResult::Move;
    } else {
        board_state.generate_legal_moves(game_state);
        return MoveResult::Generate;
    }
}
