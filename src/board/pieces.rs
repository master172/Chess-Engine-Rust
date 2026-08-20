use crate::board::move_generator::{
    bishop::Bishop, king::King, knight::Knight, pawn::Pawn, rook::Rook,
};

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

pub struct Piece {
    pub generators: &'static [MoveGenerators],
}

impl Piece {
    pub fn generate_moves(
        &self,
        index: u64,
        board_representation: &[u64; 12],
        side: &Sides,
    ) -> u64 {
        let mut generated: u64 = 0;
        for i in self.generators {
            generated = generated | i.get_moves(index, board_representation, side);
        }
        generated
    }
}

impl MoveGenerators {
    fn get_moves(&self, index: u64, board_representation: &[u64; 12], side: &Sides) -> u64 {
        match &self {
            MoveGenerators::King => King::gen_moves(index, board_representation, side),
            MoveGenerators::Bishop => Bishop::gen_moves(index, board_representation, side),
            MoveGenerators::Rook => Rook::gen_moves(index, board_representation, side),
            MoveGenerators::Knight => Knight::gen_moves(index, board_representation, side),
            MoveGenerators::Pawn => Pawn::gen_moves(index, board_representation, side),
        }
    }
}
