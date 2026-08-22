use crate::{
    board::move_generator::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, rook::Rook},
    game::GameState,
};

#[derive(PartialEq, Eq)]
pub enum MoveGenerators {
    King,
    Knight,
    Rook,
    Bishop,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Sides {
    BLACK,
    WHITE,
}

impl Sides {
    pub fn flip(&self) -> Self {
        match self {
            Self::BLACK => Self::WHITE,
            Sides::WHITE => Sides::BLACK,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Piece {
    pub generators: &'static [MoveGenerators],
}

impl Piece {
    pub fn generate_moves(
        &self,
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
        game_state: &GameState,
    ) -> u64 {
        let mut generated: u64 = 0;
        for i in self.generators {
            generated = generated | i.get_moves(index, board_representation, side, game_state);
        }
        generated
    }

    pub fn get_attacking_squares(
        &self,
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
    ) -> u64 {
        let mut generated: u64 = 0;
        for i in self.generators {
            generated = generated | i.get_attacking_squares(index, board_representation, side);
        }
        generated
    }

    pub fn pre_move_gen(
        &self,
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
        game_state: &mut GameState,
    ) {
        for i in self.generators {
            i.pre_move_gen(index, board_representation, side, game_state);
        }
    }
}

impl MoveGenerators {
    fn get_moves(
        &self,
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
        game_state: &GameState,
    ) -> u64 {
        match &self {
            MoveGenerators::King => King::gen_moves(index, board_representation, side, game_state),
            MoveGenerators::Bishop => {
                Bishop::gen_moves(index, board_representation, side, game_state)
            }
            MoveGenerators::Rook => Rook::gen_moves(index, board_representation, side, game_state),
            MoveGenerators::Knight => {
                Knight::gen_moves(index, board_representation, side, game_state)
            }
            MoveGenerators::Pawn => Pawn::gen_moves(index, board_representation, side, game_state),
        }
    }

    pub fn get_attacking_squares(
        &self,
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
    ) -> u64 {
        match &self {
            MoveGenerators::King => King::get_attacking_squares(index),
            MoveGenerators::Bishop => {
                Bishop::get_attacking_squares(index, board_representation, side)
            }
            MoveGenerators::Rook => Rook::get_attacking_squares(index, board_representation, side),
            MoveGenerators::Knight => Knight::get_attacking_squares(index),
            MoveGenerators::Pawn => Pawn::get_attacking_squares(index, side),
        }
    }

    pub fn pre_move_gen(
        &self,
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
        game_state: &mut GameState,
    ) {
        match &self {
            MoveGenerators::King => (),   //King::get_attacking_squares(index),
            MoveGenerators::Bishop => (), //{
            //    Bishop::get_attacking_squares(index, board_representation, side)
            //}
            MoveGenerators::Rook => (), //Rook::get_attacking_squares(index, board_representation, side),
            MoveGenerators::Knight => (), //Knight::get_attacking_squares(index),
            MoveGenerators::Pawn => {
                Pawn::pre_move_gen(index, board_representation, side, game_state)
            } //Pawn::get_attacking_squares(index, side),
        }
    }
}
