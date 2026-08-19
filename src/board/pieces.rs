use crate::board::move_generator::MoveGenerator;

pub enum Sides {
    BLACK,
    WHITE,
}

pub struct Piece {
    pub side: Sides,
    generators: Vec<Box<dyn MoveGenerator>>,
}
