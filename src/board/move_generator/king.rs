use crate::board::move_generator::MoveGenerator;

pub struct King {}

impl MoveGenerator for King {
    fn get_name(&self) -> String {
        String::from("King")
    }
}
