use std::collections::HashMap;

use chess::get_file_and_rank;

use crate::{
    board::{
        BB, BK, BN, BP, BQ, BR, BoardState, WB, WK, WN, WP, WQ, WR,
        move_generator::{get_all_black_pieces, get_all_white_pieces},
        pieces::Sides,
    },
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

    pub fn insufficient_material(&self, board_state: &BoardState) -> bool {
        let mut white_dark_bishop: u8 = 0;
        let mut white_light_bishop: u8 = 0;
        let mut black_dark_bishop: u8 = 0;
        let mut black_light_bishop: u8 = 0;
        let mut white_knight: u8 = 0;
        let mut black_knight: u8 = 0;

        if board_state.board_representation[WR]
            | board_state.board_representation[WQ]
            | board_state.board_representation[BR]
            | board_state.board_representation[BQ]
            | board_state.board_representation[WP]
            | board_state.board_representation[BP]
            != 0
        {
            return false;
        }

        if get_all_white_pieces(&board_state.board_representation)
            | get_all_black_pieces(&board_state.board_representation)
            == board_state.board_representation[WK] | board_state.board_representation[BK]
        {
            return true;
        }

        let mut white_bishops = board_state.board_representation[WB];
        while white_bishops != 0 {
            let index = white_bishops.trailing_zeros();
            if get_file_and_rank(index as i32).sum() % 2 == 0 {
                white_dark_bishop += 1;
            } else {
                white_light_bishop += 1;
            }
            white_bishops &= white_bishops - 1;
        }

        let mut black_bishops = board_state.board_representation[BB];
        while black_bishops != 0 {
            let index = black_bishops.trailing_zeros();
            if get_file_and_rank(index as i32).sum() % 2 == 0 {
                black_dark_bishop += 1;
            } else {
                black_light_bishop += 1;
            }
            black_bishops &= black_bishops - 1;
        }

        let mut white_knights = board_state.board_representation[WN];
        while white_knights != 0 {
            white_knight += 1;
            white_knights &= white_knights - 1;
        }

        let mut black_knights = board_state.board_representation[BN];
        while black_knights != 0 {
            black_knight += 1;
            black_knights &= black_knights - 1;
        }

        if white_dark_bishop
            + white_light_bishop
            + black_dark_bishop
            + black_light_bishop
            + white_knight
            + black_knight
            == 1
        {
            return true;
        }

        if white_dark_bishop + white_light_bishop == 1
            && black_dark_bishop + black_light_bishop == 1
        {
            if white_dark_bishop == black_dark_bishop {
                return true;
            } else if white_light_bishop == black_light_bishop {
                return true;
            }
        }
        return false;
    }
}
