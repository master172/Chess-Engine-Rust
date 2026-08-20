#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub struct FileAndRank {
    pub file: i32,
    pub rank: i32,
}

impl FileAndRank {
    pub fn new(file: i32, rank: i32) -> Self {
        Self { file, rank }
    }
    pub fn sum(&self) -> i32 {
        self.file + self.rank
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

///To parse a number to the struct for a file and rank the number must start indexing from 0
///
/// i.e this function turns 0 into a, 1 to b and so on
/// and ranks start from 0 not 1
pub fn get_file_and_rank(index: i32) -> FileAndRank {
    let file: i32 = index % 8;
    let rank: i32 = index / 8;
    FileAndRank::new(file, rank)
}

pub fn file_and_rank_to_index(file_and_rank: FileAndRank) -> i32 {
    return (file_and_rank.rank * 8) + file_and_rank.file;
}

///this function will be the standard way to convert the top-left to bottom-right orientation standard
/// to the bottom-left to top right standard,
/// this just does almost the same work as file and rank except it flips the rank
///
/// calling this on an standard index should just flip the rank
pub fn get_standard_oritentation_index(index: i32) -> i32 {
    let file: i32 = index % 8;
    let rank: i32 = 7 - (index / 8);
    return file + (rank * 8);
}

///function to convert mouse pos to an index on the board
///
/// the function just takes the mouse position, the top left corner position of the baord and the size of each square
///
/// then checks wether the click is on bounds and if it is returns `Some(index)` else returns `None`
///
/// where index is an i32 that goes from 0 to 63
pub fn mouse_pos_to_board_index(
    pos: &(f32, f32),
    start_pos: &Vector2,
    cell_size: &Vector2,
) -> Option<i32> {
    let board_width_end_pos: f32 = start_pos.x + (8.0 * cell_size.x);
    let board_height_end_pos: f32 = start_pos.y + (8.0 * cell_size.y);

    if pos.0 < start_pos.x || pos.0 > board_width_end_pos {
        return None;
    } else if pos.1 < start_pos.y || pos.1 > board_height_end_pos {
        return None;
    }
    let pos = (pos.0 - start_pos.x, pos.1 - start_pos.y);
    let file: i32 = (pos.0 / cell_size.x) as i32;
    let mut rank: i32 = (pos.1 / cell_size.y) as i32;
    rank = 56 - (rank * 8);
    return Some(file + rank);
}
