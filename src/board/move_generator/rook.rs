use crate::board::{
    move_generator::{
        SELF_SIDE_ICLUSIVE, SELF_SIDE_NON_INCLUSIVE, get_all_black_pieces, get_all_white_pieces,
        get_to_bottom, get_to_left, get_to_right, get_to_top, to_direction,
    },
    pieces::Sides,
};

pub struct Rook {}

impl Rook {
    pub fn gen_moves(index: u64, board_representation: &[u64; 12], side: &Sides) -> u64 {
        let psuedo_legal_moves: u64 =
            Self::get_psuedo_legal_moves(index, board_representation, side);
        psuedo_legal_moves
    }

    pub fn get_psuedo_legal_moves(
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

        //top check the spot 8 bits foward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                8,
                index,
                get_to_top,
                SELF_SIDE_NON_INCLUSIVE,
            )
            .unwrap();
        //left check the spot 1 bit backward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                -1,
                index,
                get_to_left,
                SELF_SIDE_NON_INCLUSIVE,
            )
            .unwrap();
        //right check the spot 1 bit foward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                1,
                index,
                get_to_right,
                SELF_SIDE_NON_INCLUSIVE,
            )
            .unwrap();
        //bottom check the spot 8 bits foward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                -8,
                index,
                get_to_bottom,
                SELF_SIDE_NON_INCLUSIVE,
            )
            .unwrap();

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

        //top check the spot 8 bits foward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                8,
                index,
                get_to_top,
                SELF_SIDE_ICLUSIVE,
            )
            .unwrap();
        //left check the spot 1 bit backward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                -1,
                index,
                get_to_left,
                SELF_SIDE_ICLUSIVE,
            )
            .unwrap();
        //right check the spot 1 bit foward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                1,
                index,
                get_to_right,
                SELF_SIDE_ICLUSIVE,
            )
            .unwrap();
        //bottom check the spot 8 bits foward to self index
        generated = generated
            | to_direction(
                my_side,
                enemy_side,
                -8,
                index,
                get_to_bottom,
                SELF_SIDE_ICLUSIVE,
            )
            .unwrap();

        generated
    }
}
