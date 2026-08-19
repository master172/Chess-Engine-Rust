use crate::board::move_generator::MoveGenerator;

pub struct Knight {}

impl MoveGenerator for Knight {
    fn get_name(&self) -> String {
        String::from("Knight")
    }
}
