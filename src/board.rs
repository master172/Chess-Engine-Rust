use crate::{
    GameState,
    board::{
        piece_definitions::{BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK},
        pieces::{
            Piece,
            Sides::{self, BLACK, WHITE},
        },
    },
};

mod move_generator;
mod piece_definitions;
mod pieces;

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

#[derive(Debug)]
pub struct BoardState {
    pub board_representation: [u64; 12],
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            board_representation: [0; 12],
        }
    }

    pub fn init_piece(&mut self, index: usize, square_index: u64) {
        self.board_representation[index] = self.board_representation[index] | 1 << square_index;
    }

    fn get_piece_from_state(&self, game_state: &mut GameState) -> Option<(Piece, Sides)> {
        let current_bit_mask: u64 = 1
            << game_state
                .current_index
                .expect("this error message shoudl never be called");
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
        let (piece, side) = self
            .get_piece_from_state(game_state)
            .expect("this is also one of those errors that should never show up");

        game_state.legal_moves = piece.generate_moves(
            game_state.current_index.unwrap() as u64,
            &self.board_representation,
            &side,
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
}

pub fn index_to_piece(index: usize) -> Option<(Piece, Sides)> {
    match index {
        WK => Some((KING, WHITE)),
        BK => Some((KING, BLACK)),
        WQ => Some((QUEEN, WHITE)),
        BQ => Some((QUEEN, BLACK)),
        WP => Some((PAWN, WHITE)),
        BP => Some((PAWN, BLACK)),
        WB => Some((BISHOP, WHITE)),
        BB => Some((BISHOP, BLACK)),
        WR => Some((ROOK, WHITE)),
        BR => Some((ROOK, BLACK)),
        WN => Some((KNIGHT, WHITE)),
        BN => Some((KNIGHT, BLACK)),
        _ => None,
    }
}
