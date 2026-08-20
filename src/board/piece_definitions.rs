use crate::board::pieces::{MoveGenerators, Piece};

pub const KING: Piece = Piece {
    generators: &[MoveGenerators::King],
};

pub const QUEEN: Piece = Piece {
    generators: &[MoveGenerators::Rook, MoveGenerators::Bishop],
};

pub const ROOK: Piece = Piece {
    generators: &[MoveGenerators::Rook],
};

pub const BISHOP: Piece = Piece {
    generators: &[MoveGenerators::Bishop],
};

pub const KNIGHT: Piece = Piece {
    generators: &[MoveGenerators::Knight],
};

pub const PAWN: Piece = Piece {
    generators: &[MoveGenerators::Pawn],
};
