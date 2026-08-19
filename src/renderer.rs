use chess::{Vector2, get_file_and_rank};
use macroquad::{color::Color, shapes::draw_rectangle};

const START_POINT: Vector2 = Vector2 {
    x: ((1280 / 2) - 64 * 6) as f32,
    y: ((720 / 2) - 64 * 5) as f32,
};
const SQUARE_SIZE: Vector2 = Vector2 { x: 64.0, y: 64.0 };

const LIGHT_CELL_COLOR: Color = Color::from_hex(0xEEEED2);
const DARK_CELL_COLOR: Color = Color::from_hex(0x769656);

pub fn render() {
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
