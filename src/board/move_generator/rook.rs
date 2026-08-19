use crate::board::move_generator::MoveGenerator;

pub struct Rook {}

impl MoveGenerator for Rook {
    fn get_name(&self) -> String {
        String::from("Rook")
    }
}
