use chess::{FileAndRank, file_and_rank_to_index, get_file_and_rank};

use crate::{
    board::{
        move_generator::{get_all_black_pieces, get_all_white_pieces},
        pieces::Sides,
    },
    game::GameState,
};

pub struct Knight {}

const REQUIRED: [(i32, i32); 8] = [
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
];
impl Knight {
    pub fn gen_moves(
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
        game_state: &GameState,
    ) -> u64 {
        let mut psuedo_legal_moves: u64 =
            Self::get_psuedo_legal_moves(index, board_representation, side, game_state);

        let pin_mask: u64;
        if (1 << index) & game_state.pin_index_mask != 0 {
            pin_mask = game_state.pin_mask[index as usize];
        } else {
            pin_mask = 0;
        }
        psuedo_legal_moves = psuedo_legal_moves & pin_mask;

        psuedo_legal_moves
    }

    pub fn get_psuedo_legal_moves(
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
        game_state: &GameState,
    ) -> u64 {
        let black_pieces: u64 = get_all_black_pieces(board_representation);
        let white_pieces: u64 = get_all_white_pieces(board_representation);

        let my_side: u64;
        let my_side_checks: u32;
        match side {
            Sides::BLACK => {
                my_side = black_pieces;
                my_side_checks = game_state.black_checks;
            }
            Sides::WHITE => {
                my_side = white_pieces;
                my_side_checks = game_state.white_checks;
            }
        };

        if my_side_checks > 1 {
            return 0;
        }
        let file_and_rank: FileAndRank = get_file_and_rank(index as i32);
        let mut generated: u64 = 0;

        for i in REQUIRED {
            generated = generated | add_pos(&file_and_rank, &i, my_side)
        }

        generated
    }

    pub fn get_attacking_squares(index: u64) -> u64 {
        let file_and_rank: FileAndRank = get_file_and_rank(index as i32);
        let mut generated: u64 = 0;
        for i in REQUIRED {
            generated = generated | add_pos_no_details(&file_and_rank, &i)
        }

        generated
    }
}
fn add_pos_no_details(file_and_rank: &FileAndRank, shift: &(i32, i32)) -> u64 {
    if !get_within_board(file_and_rank, shift) {
        return 0;
    }
    let converted_file_and_rank: FileAndRank =
        FileAndRank::new(file_and_rank.file + shift.0, file_and_rank.rank + shift.1);
    let converted_file_and_rank: u64 = file_and_rank_to_index(converted_file_and_rank) as u64;
    return 1 << converted_file_and_rank;
}

fn add_pos(file_and_rank: &FileAndRank, shift: &(i32, i32), my_side: u64) -> u64 {
    if !get_within_board(file_and_rank, shift) {
        return 0;
    }
    let converted_file_and_rank: FileAndRank =
        FileAndRank::new(file_and_rank.file + shift.0, file_and_rank.rank + shift.1);
    let converted_file_and_rank: u64 = file_and_rank_to_index(converted_file_and_rank) as u64;
    if (1 << converted_file_and_rank) & my_side != 0 {
        return 0;
    }
    return 1 << converted_file_and_rank;
}

fn get_within_board(file_and_rank: &FileAndRank, shift: &(i32, i32)) -> bool {
    return 0 <= file_and_rank.file + shift.0
        && file_and_rank.file + shift.0 <= 7
        && 0 <= file_and_rank.rank + shift.1
        && file_and_rank.rank + shift.1 <= 7;
}
