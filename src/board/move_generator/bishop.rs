use crate::board::move_generator::MoveGenerator;

pub struct Bishop {}

impl MoveGenerator for Bishop {
    fn get_name(&self) -> String {
        String::from("Bishop")
    }
}
