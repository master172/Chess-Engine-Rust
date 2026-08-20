use crate::{GameState, input::InputPackage};

mod move_generator;
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

    pub fn generate_legal_moves(&self, game_state: &mut GameState) {
        let current_bit_mask: u64 = 1
            << game_state
                .current_index
                .expect("this error message shoudl never be called");
        for (index, val) in self.board_representation.iter().enumerate() {
            if val & current_bit_mask != 0 {
                println!(
                    "{} at index {}",
                    index_to_piece_string(index as usize)
                        .expect("this error message should not appear"),
                    game_state.current_index.expect("neither should this")
                );
            }
        }
    }

    pub fn _get_all_bitboards_combined(&self) -> u64 {
        let mut final_board: u64 = 0;
        for i in self.board_representation {
            final_board = final_board | i
        }
        final_board
    }
}

pub fn index_to_piece_string(index: usize) -> Option<String> {
    match index {
        WK => Some(String::from("white king")),
        BK => Some(String::from("black king")),
        WQ => Some(String::from("white queen")),
        BQ => Some(String::from("black queen")),
        WP => Some(String::from("white pawn")),
        BP => Some(String::from("black pawn")),
        WB => Some(String::from("white bishop")),
        BB => Some(String::from("black bishop")),
        WR => Some(String::from("white rook")),
        BR => Some(String::from("black rook")),
        WN => Some(String::from("white knight")),
        BN => Some(String::from("black knight")),
        _ => None,
    }
}
