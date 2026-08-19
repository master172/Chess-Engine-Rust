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

    pub fn get_all_bitboards_combined(&self) -> u64 {
        let mut final_board: u64 = 0;
        for i in self.board_representation {
            final_board = final_board | i
        }
        final_board
    }
}
