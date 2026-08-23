pub struct DrawDetails {
    pub total_non_progressive_moves: u32,
}

impl DrawDetails {
    pub fn new() -> Self {
        Self {
            total_non_progressive_moves: 0,
        }
    }
    pub fn draw_by_excessive_non_progressive_moves(&self) -> bool {
        return self.total_non_progressive_moves > 150;
    }
}

pub struct _ZorbitHash {
    table: [[u8; 12]; 64],
}
