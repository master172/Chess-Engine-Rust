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
