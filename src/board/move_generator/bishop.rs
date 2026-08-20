use crate::board::{
    move_generator::{
        get_all_black_pieces, get_all_white_pieces, get_to_bottom_left, get_to_bottom_right,
        get_to_top_left, get_to_top_right, to_direction,
    },
    pieces::Sides,
};

pub struct Bishop {}

impl Bishop {
    pub fn gen_moves(index: u64, board_representation: &[u64; 12], side: &Sides) -> u64 {
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
        generated =
            generated | to_direction(my_side, enemy_side, 7, index, get_to_top_left).unwrap();
        //top_right check the spot 9 bits forward to self index
        generated =
            generated | to_direction(my_side, enemy_side, 9, index, get_to_top_right).unwrap();
        //bottom_left check the spot 9 bits backward to self index
        generated =
            generated | to_direction(my_side, enemy_side, -9, index, get_to_bottom_left).unwrap();
        //bottom_right check the spot 7 bits foward to self index
        generated =
            generated | to_direction(my_side, enemy_side, -7, index, get_to_bottom_right).unwrap();

        generated
    }
}
