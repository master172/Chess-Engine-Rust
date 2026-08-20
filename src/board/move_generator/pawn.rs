use crate::board::{
    move_generator::{get_all_black_pieces, get_all_white_pieces, get_to_left, get_to_right},
    pieces::Sides,
};

pub struct Pawn {}

const WHITE_STARTING_SQUARES: u64 = 0x0000_0000_0000_FF00; // White pawn starting rank (bits 8–15)
const BLACK_STARTING_SQUARES: u64 = 0x00FF_0000_0000_0000; // Black pawn starting rank (bits 48–55)
impl Pawn {
    pub fn gen_moves(index: u64, board_representation: &[u64; 12], side: &Sides) -> u64 {
        let black_pieces: u64 = get_all_black_pieces(board_representation);
        let white_pieces: u64 = get_all_white_pieces(board_representation);

        let enemy_side: u64;

        match side {
            Sides::BLACK => {
                enemy_side = white_pieces;
            }
            Sides::WHITE => {
                enemy_side = black_pieces;
            }
        };
        let mut generated: u64 = 0;
        generated = generated | add_foward_double_pos(index, side, board_representation);
        generated = generated | add_foward_pos(index, side, board_representation);
        generated = generated | add_attack_left_pos(index, side, enemy_side);
        generated = generated | add_attack_right_pos(index, side, enemy_side);

        generated
    }
}

fn add_foward_double_pos(index: u64, side: &Sides, board_representation: &[u64; 12]) -> u64 {
    let pos: u64 = get_foward_double_movement_pos(index, side);
    if validate_foward_movement(pos, board_representation) {
        return pos;
    }
    return 0;
}

fn add_foward_pos(index: u64, side: &Sides, board_representation: &[u64; 12]) -> u64 {
    let pos: u64 = get_foward_movement_pos(index, side);
    if validate_foward_movement(pos, board_representation) {
        return pos;
    }
    return 0;
}

fn get_foward_movement_pos(index: u64, side: &Sides) -> u64 {
    let pos: u64;
    match side {
        Sides::BLACK => {
            pos = 1 << (index - 8);
        }
        Sides::WHITE => {
            pos = 1 << (index + 8);
        }
    }
    pos
}

fn get_foward_double_movement_pos(index: u64, side: &Sides) -> u64 {
    let pos: u64;
    match side {
        Sides::BLACK => {
            if (1 << index) & BLACK_STARTING_SQUARES != 0 {
                pos = 1 << (index - 16);
            } else {
                pos = 0;
            }
        }
        Sides::WHITE => {
            if (1 << index) & WHITE_STARTING_SQUARES != 0 {
                pos = 1 << (index + 16);
            } else {
                pos = 0;
            }
        }
    }
    pos
}

fn validate_foward_movement(pos: u64, board_representation: &[u64; 12]) -> bool {
    let mut final_board: u64 = 0;
    for board in board_representation.iter() {
        final_board = final_board | board;
    }
    if pos & final_board != 0 {
        return false;
    }
    return true;
}

fn add_attack_left_pos(index: u64, side: &Sides, enemy_board: u64) -> u64 {
    let pos: u64 = get_attack_left_pos(index, side);
    if validate_attack_pos(pos, enemy_board) {
        return pos;
    }
    return 0;
}

fn add_attack_right_pos(index: u64, side: &Sides, enemy_board: u64) -> u64 {
    let pos: u64 = get_attack_right_pos(index, side);
    if validate_attack_pos(pos, enemy_board) {
        return pos;
    }
    return 0;
}

fn validate_attack_pos(pos: u64, enemy_board: u64) -> bool {
    // you could add en passant here to a bit wise or with the en passant mask here
    // and remember each bit in the en passant mask maps to only 1 bit where the deletion can happen
    // let mut final_board: u64 = 0;
    if pos & enemy_board != 0 {
        return true;
    }
    return false;
}

fn get_attack_left_pos(index: u64, side: &Sides) -> u64 {
    if get_to_left(index) == 0 {
        return 0;
    }
    let attack_pos: u64;
    match side {
        Sides::WHITE => attack_pos = 1 << (index + 7),
        Sides::BLACK => attack_pos = 1 << (index - 9),
    }
    attack_pos
}

fn get_attack_right_pos(index: u64, side: &Sides) -> u64 {
    if get_to_right(index) == 0 {
        return 0;
    }
    let attack_pos: u64;
    match side {
        Sides::WHITE => attack_pos = 1 << (index + 9),
        Sides::BLACK => attack_pos = 1 << (index - 7),
    }
    attack_pos
}
