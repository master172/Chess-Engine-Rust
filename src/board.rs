use crate::{
    GameState,
    board::{
        move_generator::king::King,
        piece_definitions::{BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK},
        pieces::{
            Piece,
            Sides::{self, BLACK, WHITE},
        },
    },
};

mod move_generator;
mod piece_definitions;
pub mod pieces;

///index of white king
pub const WK: usize = 0;

///index of black king
pub const BK: usize = 1;

///index of white queen
pub const WQ: usize = 2;

///index of black queen
pub const BQ: usize = 3;

///index of white pawns
pub const WP: usize = 4;

///index of black pawns
pub const BP: usize = 5;

///index of white knights
pub const WN: usize = 6;

///index of black knights
pub const BN: usize = 7;

///index of white rooks
pub const WR: usize = 8;

///index of black rooks
pub const BR: usize = 9;

///index of white bishops
pub const WB: usize = 10;

///index of black bishops
pub const BB: usize = 11;

pub const BLACK_INDEXES: [usize; 6] = [BK, BQ, BN, BR, BB, BP];
pub const WHITE_INDEXES: [usize; 6] = [WK, WQ, WN, WR, WB, WP];

#[derive(Debug)]
pub struct BoardState {
    pub board_representation: [u64; 12],
    pub side_to_start: Sides,
}

impl BoardState {
    pub fn new(side: Sides) -> Self {
        Self {
            board_representation: [0; 12],
            side_to_start: side,
        }
    }

    pub fn init_piece(&mut self, index: usize, square_index: u64) {
        self.board_representation[index] = self.board_representation[index] | 1 << square_index;
    }

    pub fn get_piece_from_index(&self, index: i32) -> Option<(Piece, Sides, usize)> {
        let current_bit_mask: u64 = 1 << index;
        for (index, val) in self.board_representation.iter().enumerate() {
            if val & current_bit_mask != 0 {
                return index_to_piece(index);
            }
        }
        return None;
    }

    pub fn generate_legal_moves(&self, game_state: &mut GameState) {
        if !self.validate_piece_selection(
            game_state
                .current_index
                .expect("again this error message should also never show up") as u64,
        ) {
            return;
        }
        game_state.previous_index = Some(game_state.current_index.unwrap());
        let (piece, side, index) = self
            .get_piece_from_index(game_state.current_index.unwrap())
            .expect("this is also one of those errors that should never show up");
        if game_state.dev_mode == false && side != game_state.current_side {
            return;
        }
        game_state.current_array_index = Some(index);

        game_state.legal_moves = piece.generate_moves(
            game_state.current_index.unwrap() as u64,
            &self.board_representation,
            &side,
            &game_state,
        );
    }

    pub fn validate_piece_selection(&self, index: u64) -> bool {
        for i in self.board_representation {
            if i & (1 << index) != 0 {
                return true;
            }
        }
        return false;
    }

    pub fn reset_necessary_game_state_variables(&self, game_state: &mut GameState) {
        game_state.legal_moves = 0;
        game_state.previous_index = None;
        game_state.current_index = None;
        game_state.current_array_index = None
    }

    pub fn valid_piece_selection(&self, index: i32, game_state: &GameState) -> bool {
        let (_, side, _) = self.get_piece_from_index(index).unwrap();
        return side == game_state.current_side;
    }

    pub fn move_piece(&mut self, game_state: &mut GameState) {
        let (_, side, _) = self
            .get_piece_from_index(game_state.previous_index.unwrap())
            .unwrap();
        self.board_representation[game_state.current_array_index.unwrap()] &=
            !(1 << (game_state.previous_index.unwrap() as u64));
        self.board_representation[game_state.current_array_index.unwrap()] |=
            1 << (game_state.current_index.unwrap() as u64);
        let capture_mask: u64 = !(1 << (game_state.current_index.unwrap() as u64));
        match side {
            Sides::WHITE => {
                for i in BLACK_INDEXES {
                    self.board_representation[i] &= capture_mask;
                }
            }
            Sides::BLACK => {
                for i in WHITE_INDEXES {
                    self.board_representation[i] &= capture_mask;
                }
            }
        }

        self.set_attacked_squares(side, game_state);
        self.handle_king_saftey(side.flip(), game_state);

        self.reset_necessary_game_state_variables(game_state);
        if game_state.dev_mode == false {
            game_state.current_side = game_state.current_side.flip();
        }
    }

    // the side passed to it must be the side that is playing currently
    pub fn handle_king_saftey(&self, side: Sides, game_state: &mut GameState) {
        let king_index: usize = if side == Sides::WHITE { WK } else { BK };
        let mut king_mask: u64 = self.board_representation[king_index];
        while king_mask != 0 {
            let index = king_mask.trailing_zeros();
            King::king_saftey(index as u64, side, game_state, &self.board_representation);
            king_mask &= king_mask - 1
        }
    }

    // the side provided to this function must be the other side i.e the side that is not moving now
    pub fn set_attacked_squares(&self, side: Sides, game_state: &mut GameState) {
        let current_indexes: [usize; 6] = if side == Sides::WHITE {
            WHITE_INDEXES
        } else {
            BLACK_INDEXES
        };
        let current_attacks: &mut u64 = if side == Sides::WHITE {
            &mut game_state.white_attacked
        } else {
            &mut game_state.black_attacked
        };
        *current_attacks = 0;
        for board_index in current_indexes {
            let mut mask: u64 = self.board_representation[board_index];
            while mask != 0 {
                let index = mask.trailing_zeros();
                let (piece, side, _) = index_to_piece(board_index).unwrap();

                *current_attacks |=
                    piece.get_attacking_squares(index as u64, &self.board_representation, &side);
                mask &= mask - 1;
            }
        }
    }
}

pub fn index_to_piece(index: usize) -> Option<(Piece, Sides, usize)> {
    match index {
        WK => Some((KING, WHITE, index)),
        BK => Some((KING, BLACK, index)),
        WQ => Some((QUEEN, WHITE, index)),
        BQ => Some((QUEEN, BLACK, index)),
        WP => Some((PAWN, WHITE, index)),
        BP => Some((PAWN, BLACK, index)),
        WB => Some((BISHOP, WHITE, index)),
        BB => Some((BISHOP, BLACK, index)),
        WR => Some((ROOK, WHITE, index)),
        BR => Some((ROOK, BLACK, index)),
        WN => Some((KNIGHT, WHITE, index)),
        BN => Some((KNIGHT, BLACK, index)),
        _ => None,
    }
}
