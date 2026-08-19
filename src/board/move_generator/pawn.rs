use crate::board::move_generator::MoveGenerator;

pub struct Pawn {}

impl MoveGenerator for Pawn {
    fn get_name(&self) -> String {
        String::from("Pawn")
    }
}
