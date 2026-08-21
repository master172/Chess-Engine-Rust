use crate::{
    board::{
        BB, BN, BP, BQ, BR, WB, WN, WP, WQ, WR,
        move_generator::{
            get_all_black_pieces, get_all_white_pieces, get_to_bottom, get_to_bottom_left,
            get_to_bottom_right, get_to_left, get_to_right, get_to_top, get_to_top_left,
            get_to_top_right,
        },
        pieces::Sides,
    },
    game::GameState,
    lookup_helpers::{
        BLACK_PAWN_ATTACK_REFERENCE, KNIGHT_ATTACK_REFERENCE, WHITE_PAWN_ATTACK_REFERENCE,
    },
};

pub struct King {}

const CARDINAL_RELEVANT_BLACKS: [[usize; 2]; 8] = [
    [BB, BQ],
    [BR, BQ],
    [BB, BQ],
    [BR, BQ],
    [BR, BQ],
    [BB, BQ],
    [BR, BQ],
    [BB, BQ],
];
const CARDINAL_RELEVANT_WHITES: [[usize; 2]; 8] = [
    [WB, WQ],
    [WR, WQ],
    [WB, WQ],
    [WR, WQ],
    [WR, WQ],
    [WB, WQ],
    [WR, WQ],
    [WB, WQ],
];
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
            Self::get_psuedo_legal_moves(index, board_representation, side, game_state);
        let legal_moves: u64 = Self::safe_square_gen(psuedo_legal_moves, side, game_state);
        legal_moves
    }

    /// the most important function this is the function that allows the seperation from psuedo legal moves to legal moves
    /// this is done by keeping king saftey in mind while doing each move and the knig saftey associated values are to be configured
    /// by the king
    /// important here index does not represent its bitmask position but rather its index on the board
    pub fn king_saftey(
        index: u64,
        side: Sides,
        game_state: &mut GameState,
        board_representation: &[u64; 12],
    ) {
        // for the pre setup turn the index 0 to 63 to a bit mask position
        //let bitmask_pos: u64 = 1 << index;
        // first start with declaring all necessary side dependent variables
        let checking_value: &mut u32;
        let mut pawn_map: u64;
        let pawn_index: usize;
        let knight_index: usize;
        let evasion_mask: &mut u64;
        let blockers_mask: u64;
        let relevant_attackers: &[[usize; 2]; 8];
        let my_mask: &mut u64;
        //then configure them
        if side == Sides::WHITE {
            checking_value = &mut game_state.white_checks;
            pawn_index = BP;
            knight_index = BN;
            pawn_map = BLACK_PAWN_ATTACK_REFERENCE[index as usize];
            evasion_mask = &mut game_state.white_evasion_mask;
            relevant_attackers = &CARDINAL_RELEVANT_BLACKS;
            blockers_mask = get_all_white_pieces(board_representation);
            my_mask = &mut game_state.white_king_mask;
        } else {
            checking_value = &mut game_state.black_checks;
            pawn_index = WP;
            knight_index = WN;
            pawn_map = WHITE_PAWN_ATTACK_REFERENCE[index as usize];
            evasion_mask = &mut game_state.black_evasion_mask;
            relevant_attackers = &CARDINAL_RELEVANT_WHITES;
            blockers_mask = get_all_black_pieces(board_representation);
            my_mask = &mut game_state.black_king_mask;
        };
        // it is also necessary that we reset game state variables before calculating them
        // since we do a full calc each time we can just reset all of them no problem
        game_state.pin_mask = [0; 64];
        game_state.pin_index_mask = 0;
        *evasion_mask = 0;
        *checking_value = 0;
        *my_mask = 0;
        // first we start with non slider checks

        // starting with pawns
        while pawn_map != 0 {
            let index: u64 = pawn_map.trailing_zeros() as u64;
            if (1 << index) & board_representation[pawn_index] != 0 {
                *checking_value += 1;
                *evasion_mask |= (1 << index) as u64;
            }
            pawn_map &= pawn_map - 1;
        }

        //now for knights
        let mut knight_map: u64 = KNIGHT_ATTACK_REFERENCE[index as usize];
        while knight_map != 0 {
            let index: u64 = knight_map.trailing_zeros() as u64;
            if (1 << index) & board_representation[knight_index] != 0 {
                *checking_value += 1;
                *evasion_mask |= (1 << index) as u64;
            }
            knight_map &= knight_map - 1;
        }

        // this should handle all checks of non sliding pieces these checks are special since no pins can be made
        // and the evasion mask is simply the checking pieces position
        // the next part is purely for checks from sliding pieces

        // start with looping over all 8 cardinal checks and using their return value as an upper bound
        for (check_index, check) in CARDINAL_CHECKS.iter().enumerate() {
            let max_dist = check(index);
            // now loop as many times as the max dist
            // we establish a pinned_piece_mask here which will only be actually used on an actual pin
            // otherwise it serves as a way to see if we have met another piece of the same side before or not
            let mut pin_mask: u64 = 0;
            let mut acquired_mask: u64 = 0;
            let relevant_attackers_array: [usize; 2] = relevant_attackers[check_index];
            //this is the loop that projects a ray in each direction
            'ray_loop: for i in 1..=max_dist {
                let target_bit_mask: u64 =
                    1 << (index as i32 + (CARDINAL_SHIFTS[check_index] * i as i32)) as i32;
                acquired_mask |= target_bit_mask;

                //this loop handles all relevant attackers in an array all checks pinning and evasion
                for possible_attackers_index in relevant_attackers_array {
                    if board_representation[possible_attackers_index] & target_bit_mask != 0 {
                        if pin_mask != 0 {
                            game_state.pin_index_mask |= pin_mask;
                            game_state.pin_mask[target_bit_mask.trailing_zeros() as usize] =
                                acquired_mask;
                            break 'ray_loop;
                        } else {
                            *my_mask |= 1 << (index as i32 - CARDINAL_SHIFTS[check_index]);
                            *checking_value += 1;
                            *evasion_mask |= acquired_mask;
                            break 'ray_loop;
                        }
                    }
                }

                // comparitevly handling blockers is simple since we do not care about the type of blocker
                if target_bit_mask & blockers_mask != 0 {
                    if pin_mask != 0 {
                        break 'ray_loop;
                    } else {
                        pin_mask = target_bit_mask;
                    }
                }
            }
        }
    }

    pub fn safe_square_gen(psuedo_legal_moves: u64, side: &Sides, game_state: &GameState) -> u64 {
        let mut dangerous_squares: u64 = 0;
        match side {
            Sides::WHITE => dangerous_squares |= game_state.black_attacked,
            Sides::BLACK => dangerous_squares |= game_state.white_attacked,
        };
        psuedo_legal_moves & !dangerous_squares
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
        let my_mask: u64;
        match side {
            Sides::BLACK => {
                my_side = black_pieces;
                my_mask = game_state.black_king_mask;
            }
            Sides::WHITE => {
                my_side = white_pieces;
                my_mask = game_state.white_king_mask;
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
        generated &= !my_mask;
        generated
    }

    pub fn get_attacking_squares(index: u64) -> u64 {
        let mut generated: u64 = 0;
        for i in 0..8 {
            if CARDINAL_CHECKS[i](index) == 0 {
                continue;
            } else {
                generated |= 1 << (index as i32 + CARDINAL_SHIFTS[i])
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
