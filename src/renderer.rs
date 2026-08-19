use chess::{FileAndRank, Vector2, get_file_and_rank};
use macroquad::{
    color::{BLACK, Color, WHITE},
    math::vec2,
    shapes::draw_rectangle,
    texture::{DrawTextureParams, Texture2D, draw_texture, draw_texture_ex, load_texture},
    ui::widgets::Texture,
    window::clear_background,
};

pub const START_POINT: Vector2 = Vector2 {
    x: ((1280 / 2) - 64 * 6) as f32,
    y: ((720 / 2) - 64 * 5) as f32,
};
pub const SQUARE_SIZE: Vector2 = Vector2 { x: 64.0, y: 64.0 };

pub const LIGHT_CELL_COLOR: Color = Color::from_hex(0xEEEED2);
pub const DARK_CELL_COLOR: Color = Color::from_hex(0x769656);

pub fn render() {
    clear_background(BLACK);
    let mut current_point: Vector2 = START_POINT;
    for i in 0..64 {
        increment_current_point(i, &mut current_point);
        draw_rectangle(
            current_point.x,
            current_point.y,
            SQUARE_SIZE.x,
            SQUARE_SIZE.y,
            get_current_color(i),
        );
    }
}

fn standard_board_index_to_texture_pos(index: u64) -> Vector2 {
    let board_start_pos_x: f32 = START_POINT.x;
    let board_start_pos_y: f32 = START_POINT.y + (7.0 * SQUARE_SIZE.y);
    let files_and_rank: FileAndRank = get_file_and_rank(index as i32);
    let x_pos: f32 = board_start_pos_x + (SQUARE_SIZE.x * files_and_rank.file as f32);
    let y_pos: f32 = board_start_pos_y - (SQUARE_SIZE.y * files_and_rank.rank as f32);
    Vector2::new(x_pos, y_pos)
}

pub fn draw_texture_at_pos(index: u64, texture: &Texture2D) {
    assert!(index < 63, "index must be lesser than 63 to draw the piece");
    let position_to_render: Vector2 = standard_board_index_to_texture_pos(index);
    let draw_params: DrawTextureParams = DrawTextureParams {
        dest_size: Some(vec2(SQUARE_SIZE.x, SQUARE_SIZE.y)),
        ..Default::default()
    };
    draw_texture_ex(
        texture,
        position_to_render.x,
        position_to_render.y,
        WHITE,
        draw_params,
    );
}
fn increment_current_point(i: i32, point: &mut Vector2) {
    if i == 0 {
        return;
    }
    if i % 8 == 0 {
        point.x = START_POINT.x;
        point.y += SQUARE_SIZE.y;
    } else {
        point.x += SQUARE_SIZE.x;
    }
}

fn get_current_color(i: i32) -> Color {
    return if get_file_and_rank(i).sum() % 2 == 0 {
        LIGHT_CELL_COLOR
    } else {
        DARK_CELL_COLOR
    };
}
