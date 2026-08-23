use std::collections::HashMap;

use crate::{
    board::{BoardState, pieces::Sides},
    game::GameState,
    zorbist_keys::{CASTLING_KEYS, EN_PASSANT_KEYS, PIECE_KEYS, SIDE_KEY},
};

pub struct DrawDetails {
    pub total_non_progressive_moves: u32,

    pub zorbist_hash: HashMap<u64, u8>,
}

impl DrawDetails {
    pub fn new(game_state: &GameState, board_state: &BoardState) -> Self {
        let mut result = Self {
            total_non_progressive_moves: 0,
            zorbist_hash: HashMap::new(),
        };
        result.add_zorbist_hash(board_state.side_to_start, game_state, board_state);
        result
    }
    pub fn draw_by_excessive_non_progressive_moves(&self) -> bool {
        return self.total_non_progressive_moves > 150;
    }

    pub fn add_zorbist_hash(
        &mut self,
        side: Sides,
        game_state: &GameState,
        board_state: &BoardState,
    ) -> u8 {
        let mut hash: u64 = 0;
        for i in 0..12 {
            let mut board = board_state.board_representation[i];
            while board != 0 {
                let index = board.trailing_zeros();
                hash ^= PIECE_KEYS[i][index as usize];
                board &= board - 1;
            }
        }
        if side == Sides::BLACK {
            hash ^= SIDE_KEY;
        }

        hash ^= CASTLING_KEYS[game_state.castling_rights as usize];

        if game_state.en_passant_mask != 0
            && game_state.en_passant_mask & game_state.current_legal_move_mask != 0
        {
            let square_index = game_state.en_passant_mask.trailing_zeros();
            hash ^= EN_PASSANT_KEYS[(square_index % 8) as usize];
        }

        let count = self.zorbist_hash.entry(hash).or_insert(0);
        *count += 1;

        *count
    }
}
