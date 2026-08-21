use crate::{
    board::{
        move_generator::{
            get_all_black_pieces, get_all_white_pieces, get_to_bottom, get_to_bottom_left,
            get_to_bottom_right, get_to_left, get_to_right, get_to_top, get_to_top_left,
            get_to_top_right,
        },
        pieces::Sides,
    },
    game::GameState,
};

pub struct King {}

const CARDINAL_SHIFTS: [i32; 8] = [7, 8, 9, -1, 1, -9, -8, -7];
const CARDINAL_CHECKS: [fn(u64) -> usize; 8] = [
    get_to_top_left,
    get_to_top,
    get_to_top_right,
    get_to_left,
    get_to_right,
    get_to_bottom_left,
    get_to_bottom,
    get_to_bottom_right,
];

impl King {
    pub fn gen_moves(
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
        game_state: &GameState,
    ) -> u64 {
        let psuedo_legal_moves: u64 =
            Self::get_psuedo_legal_moves(index, board_representation, side);
        let legal_moves: u64 = Self::king_saftey(psuedo_legal_moves, side, game_state);
        legal_moves
    }

    pub fn king_saftey(psuedo_legal_moves: u64, side: &Sides, game_state: &GameState) -> u64 {
        let mut dangerous_squares: u64 = 0;
        match side {
            Sides::WHITE => dangerous_squares |= game_state.black_attacked,
            Sides::BLACK => dangerous_squares |= game_state.white_aattacked,
        };
        psuedo_legal_moves & !dangerous_squares
    }

    pub fn get_psuedo_legal_moves(
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
    ) -> u64 {
        let black_pieces: u64 = get_all_black_pieces(board_representation);
        let white_pieces: u64 = get_all_white_pieces(board_representation);

        let my_side: u64;

        match side {
            Sides::BLACK => {
                my_side = black_pieces;
            }
            Sides::WHITE => {
                my_side = white_pieces;
            }
        };
        let mut generated: u64 = 0;
        for i in 0..8 {
            if CARDINAL_CHECKS[i](index) == 0 {
                continue;
            } else {
                generated |= add_pos(index, CARDINAL_SHIFTS[i], my_side)
            }
        }
        generated
    }
}

fn add_pos(index: u64, shift: i32, my_side: u64) -> u64 {
    if !validate_pos(index as i32 + shift, my_side) {
        return 0;
    }
    return 1 << (index as i32 + shift);
}

fn validate_pos(index: i32, my_side: u64) -> bool {
    return (1 << index) & my_side == 0;
}
