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

pub enum Sides {
    BLACK,
    WHITE,
}

pub struct Piece {
    pub generators: &'static [MoveGenerators],
}

impl Piece {
    pub fn generate_moves(&self) -> u64 {
        let mut generated: u64 = 0;
        for i in self.generators {
            generated = generated | i.get_moves();
        }
        generated
    }
}

impl MoveGenerators {
    fn get_moves(&self) -> u64 {
        match &self {
            MoveGenerators::King => King::gen_moves(),
            MoveGenerators::Bishop => Bishop::gen_moves(),
            MoveGenerators::Rook => Rook::gen_moves(),
            MoveGenerators::Knight => Knight::gen_moves(),
            MoveGenerators::Pawn => Pawn::gen_moves(),
        }
    }
}
