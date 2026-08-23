use crate::{
    board::{BP, BoardResult, BoardState, WP, pieces::Sides},
    draw_manager::DrawDetails,
    game::MoveResult::DrawByRepition,
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
    pub all_legal_moves: [u64; 64],
    pub current_legal_move_mask: u64,
}

pub enum MoveResult {
    Idle,
    Move,
    Generate,
    Promotion(Sides),
    CheckMate(Sides),
    StaleMate(Sides),
    DrawByRepition,
}

impl GameState {
    pub fn new(dev_mode: bool, side: Sides, castling_rights: u8) -> Self {
        Self {
            current_index: None,
            legal_moves: 0,
            all_legal_moves: [0; 64],
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
            current_legal_move_mask: 0,
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
    draw_details: &mut DrawDetails,
    //input: &InputPackage,
) -> MoveResult {
    //first check and handle pawn promotion

    //then the regular gameplay
    if game_state.current_index.is_none() {
        return MoveResult::Idle;
    }
    if game_state.legal_moves != 0
        && (1 << game_state.current_index.unwrap()) & game_state.legal_moves != 0
    {
        let result = match board_state.move_piece(game_state) {
            BoardResult::None => {
                draw_details.total_non_progressive_moves += 1;
                MoveResult::Move
            }
            BoardResult::Promotion(side) => MoveResult::Promotion(side),
            BoardResult::CheckMate(side) => MoveResult::CheckMate(side),
            BoardResult::StaleMate(side) => MoveResult::StaleMate(side),
            BoardResult::Capture => {
                draw_details.total_non_progressive_moves = 0;
                MoveResult::Move
            }
            BoardResult::PawnMove => {
                draw_details.total_non_progressive_moves = 0;
                MoveResult::Move
            }
        };

        let num_appearence: u8 =
            draw_details.add_zorbist_hash(game_state.current_side, game_state, board_state);

        if num_appearence >= 3 {
            return DrawByRepition;
        }

        result
    } else {
        board_state.generate_legal_moves(game_state);
        return MoveResult::Generate;
    }
}

pub fn handle_promotion(
    game_state: &mut GameState,
    board_state: &mut BoardState,
    given_index: usize, //input: &InputPackage,
) -> BoardResult {
    if game_state.black_promotion_mask != 0 {
        board_state.board_representation[BP] &= !game_state.black_promotion_mask;
        board_state.board_representation[given_index] |= game_state.black_promotion_mask;
        game_state.black_promotion_mask = 0;
        return handle_promotion_after_effects(Sides::BLACK, board_state, game_state);
    }
    if game_state.white_promotion_mask != 0 {
        board_state.board_representation[WP] &= !game_state.white_promotion_mask;
        board_state.board_representation[given_index] |= game_state.white_promotion_mask;
        game_state.white_promotion_mask = 0;
        return handle_promotion_after_effects(Sides::WHITE, board_state, game_state);
    }
    return BoardResult::None;
}

pub fn handle_promotion_after_effects(
    side: Sides,
    board_state: &mut BoardState,
    game_state: &mut GameState,
) -> BoardResult {
    board_state.set_attacked_squares(side, game_state);
    board_state.handle_king_saftey(side.flip(), game_state);
    board_state.gen_all_legal_moves(game_state, side.flip());
    return board_state.check_endgame(side.flip(), game_state);
}
