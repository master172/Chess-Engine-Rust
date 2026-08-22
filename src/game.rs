use crate::{
    board::{BP, BQ, BoardResult, BoardState, WP, WQ, pieces::Sides},
    input::InputPackage,
};

pub struct GameState {
    //setup values
    pub dev_mode: bool,
    pub current_side: Sides,

    //king saftey values
    pub black_attacked: u64,
    pub white_attacked: u64,

    pub white_checks: u32,
    pub black_checks: u32,

    pub white_evasion_mask: u64,
    pub black_evasion_mask: u64,

    pub white_king_mask: u64,
    pub black_king_mask: u64,

    pub white_enemy_blockers: u64,
    pub black_enemy_blockers: u64,

    pub pin_index_mask: u64,
    pub pin_mask: [u64; 64],

    //special move info
    pub en_passant_candidate_mask: u64,
    pub en_passant_mask: u64,
    pub en_passant_capture_mask: u64,

    pub castling_rights: u8,

    pub black_promotion_mask: u64,
    pub white_promotion_mask: u64,
    //move gen values
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
    pub fn new(dev_mode: bool, side: Sides, castling_rights: u8) -> Self {
        Self {
            current_index: None,
            legal_moves: 0,
            previous_index: None,
            current_array_index: None,
            dev_mode,
            black_attacked: 0,
            white_attacked: 0,
            current_side: side,
            white_checks: 0,
            black_checks: 0,
            white_evasion_mask: 0,
            black_evasion_mask: 0,
            pin_index_mask: 0,
            pin_mask: [0; 64],
            white_king_mask: 0,
            black_king_mask: 0,
            en_passant_candidate_mask: 0,
            en_passant_capture_mask: 0,
            en_passant_mask: 0,
            white_enemy_blockers: 0,
            black_enemy_blockers: 0,
            castling_rights,
            black_promotion_mask: 0,
            white_promotion_mask: 0,
        }
    }

    pub fn input_to_game_state(&mut self, input: &mut InputPackage) {
        match input.left_mouse_index {
            Some(val) => self.current_index = Some(val),
            None => self.current_index = None,
        }
    }
}

pub fn handle_game_state(
    game_state: &mut GameState,
    board_state: &mut BoardState,
    input: &InputPackage,
) -> MoveResult {
    //first check and handle pawn promotion

    //then the regular gameplay
    if game_state.current_index.is_none() {
        return MoveResult::Idle;
    }
    if game_state.legal_moves != 0
        && (1 << game_state.current_index.unwrap()) & game_state.legal_moves != 0
    {
        match board_state.move_piece(game_state) {
            BoardResult::None => (),
            BoardResult::Promotion => handle_promotion(game_state, board_state, input),
        }
        return MoveResult::Move;
    } else {
        board_state.generate_legal_moves(game_state);
        return MoveResult::Generate;
    }
}

fn handle_promotion(
    game_state: &mut GameState,
    board_state: &mut BoardState,
    input: &InputPackage,
) {
    if game_state.black_promotion_mask != 0 {
        let given_index: usize = BQ;
        board_state.board_representation[BP] &= !game_state.black_promotion_mask;
        board_state.board_representation[given_index] |= game_state.black_promotion_mask;
        game_state.black_promotion_mask = 0;
    }
    if game_state.white_promotion_mask != 0 {
        let given_index: usize = WQ;
        board_state.board_representation[WP] &= !game_state.white_promotion_mask;
        board_state.board_representation[given_index] |= game_state.white_promotion_mask;
        game_state.white_promotion_mask = 0;
    }
}
