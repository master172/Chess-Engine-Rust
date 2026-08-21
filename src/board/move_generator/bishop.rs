use crate::{
    board::{
        move_generator::{
            SELF_SIDE_INCLUSIVE, SELF_SIDE_NON_INCLUSIVE, get_all_black_pieces,
            get_all_white_pieces, get_to_bottom_left, get_to_bottom_right, get_to_top_left,
            get_to_top_right, to_direction,
        },
        pieces::Sides,
    },
    game::GameState,
};

pub struct Bishop {}

impl Bishop {
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
            pin_mask = !0;
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
        let enemy_side: u64;

        let my_side_checks: u32;
        let my_side_evasion_mask: u64;

        match side {
            Sides::BLACK => {
                my_side = black_pieces;
                enemy_side = white_pieces;
                my_side_checks = game_state.black_checks;
                my_side_evasion_mask = game_state.black_evasion_mask;
            }
            Sides::WHITE => {
                my_side = white_pieces;
                enemy_side = black_pieces;
                my_side_checks = game_state.white_checks;
                my_side_evasion_mask = game_state.white_evasion_mask;
            }
        };

        if my_side_checks > 1 {
            return 0;
        }
        let mut generated: u64 = 0;

        //top_left check the spot 7 bits foward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                7,
                index,
                get_to_top_left,
                SELF_SIDE_NON_INCLUSIVE,
            )
            .unwrap();
        //top_right check the spot 9 bits forward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                9,
                index,
                get_to_top_right,
                SELF_SIDE_NON_INCLUSIVE,
            )
            .unwrap();
        //bottom_left check the spot 9 bits backward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                -9,
                index,
                get_to_bottom_left,
                SELF_SIDE_NON_INCLUSIVE,
            )
            .unwrap();
        //bottom_right check the spot 7 bits foward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                -7,
                index,
                get_to_bottom_right,
                SELF_SIDE_NON_INCLUSIVE,
            )
            .unwrap();

        if my_side_checks == 1 {
            generated &= my_side_evasion_mask;
        }
        generated
    }

    pub fn get_attacking_squares(
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
    ) -> u64 {
        let black_pieces: u64 = get_all_black_pieces(board_representation);
        let white_pieces: u64 = get_all_white_pieces(board_representation);

        let my_side: u64;
        let enemy_side: u64;

        match side {
            Sides::BLACK => {
                my_side = black_pieces;
                enemy_side = white_pieces;
            }
            Sides::WHITE => {
                my_side = white_pieces;
                enemy_side = black_pieces;
            }
        };
        let mut generated: u64 = 0;

        //top_left check the spot 7 bits foward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                7,
                index,
                get_to_top_left,
                SELF_SIDE_INCLUSIVE,
            )
            .unwrap();
        //top_right check the spot 9 bits forward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                9,
                index,
                get_to_top_right,
                SELF_SIDE_INCLUSIVE,
            )
            .unwrap();
        //bottom_left check the spot 9 bits backward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                -9,
                index,
                get_to_bottom_left,
                SELF_SIDE_INCLUSIVE,
            )
            .unwrap();
        //bottom_right check the spot 7 bits foward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                -7,
                index,
                get_to_bottom_right,
                SELF_SIDE_INCLUSIVE,
            )
            .unwrap();

        generated
    }
}
