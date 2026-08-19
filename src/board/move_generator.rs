mod bishop;
mod king;
mod knight;
mod pawn;
mod rook;

pub trait MoveGenerator {
    fn get_name(&self) -> String;
}
