use chess::mouse_pos_to_board_index;
use macroquad::input::MouseButton;
use macroquad::input::is_mouse_button_pressed;
use macroquad::input::mouse_position;

use crate::renderer::SQUARE_SIZE;
use crate::renderer::START_POINT;

pub fn gather_input() {
    if is_mouse_button_pressed(MouseButton::Left) {
        let val = mouse_pos_to_board_index(&mouse_position(), &START_POINT, &SQUARE_SIZE);
        match val {
            Some(val) => println!("click at index {val}"),
            None => (),
        }
    }
}
