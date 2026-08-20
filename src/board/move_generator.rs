use crate::board::{BB, BK, BN, BP, BQ, BR, WB, WK, WN, WP, WQ, WR};
use chess::{FileAndRank, get_file_and_rank};
use std::cmp::min;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod rook;

pub fn get_all_black_pieces(board_representation: &[u64; 12]) -> u64 {
    return board_representation[BK]
        | board_representation[BQ]
        | board_representation[BB]
        | board_representation[BR]
        | board_representation[BN]
        | board_representation[BP];
}

pub fn get_all_white_pieces(board_representation: &[u64; 12]) -> u64 {
    return board_representation[WK]
        | board_representation[WQ]
        | board_representation[WB]
        | board_representation[WR]
        | board_representation[WN]
        | board_representation[WP];
}

fn to_direction(
    my_side: u64,
    enemy_side: u64,
    increment_counter: i32,
    index: u64,
    max_function: fn(u64) -> usize,
) -> Option<u64> {
    let mut generated: u64 = 0;
    let current_bit_mask: u64 = 1 << index;
    for i in 1..=max_function(index) {
        let target_bit_mask: u64;
        if increment_counter < 0 {
            target_bit_mask = current_bit_mask >> increment_counter.abs() as usize * i;
        } else if increment_counter > 0 {
            target_bit_mask = current_bit_mask << increment_counter.abs() as usize * i;
        } else {
            return None;
        }
        if target_bit_mask & enemy_side != 0 {
            generated = generated | target_bit_mask;
            break;
        } else if target_bit_mask & my_side != 0 {
            break;
        } else {
            generated = generated | target_bit_mask
        }
    }

    return Some(generated);
}

pub fn get_to_top_right(index: u64) -> usize {
    return min(get_to_top(index), get_to_right(index));
}

pub fn get_to_top_left(index: u64) -> usize {
    return min(get_to_top(index), get_to_left(index));
}

pub fn get_to_bottom_right(index: u64) -> usize {
    return min(get_to_bottom(index), get_to_right(index));
}

pub fn get_to_bottom_left(index: u64) -> usize {
    return min(get_to_bottom(index), get_to_left(index));
}

pub fn get_to_right(index: u64) -> usize {
    let file_and_rank: FileAndRank = get_file_and_rank(index as i32);
    return (7 - file_and_rank.file) as usize;
}

pub fn get_to_left(index: u64) -> usize {
    let file_and_rank: FileAndRank = get_file_and_rank(index as i32);
    return file_and_rank.file as usize;
}

pub fn get_to_top(index: u64) -> usize {
    let file_and_rank: FileAndRank = get_file_and_rank(index as i32);
    return (7 - file_and_rank.rank) as usize;
}

pub fn get_to_bottom(index: u64) -> usize {
    let file_and_rank: FileAndRank = get_file_and_rank(index as i32);
    return file_and_rank.rank as usize;
}
