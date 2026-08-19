use chess::mouse_pos_to_board_index;
use macroquad::input::MouseButton;
use macroquad::input::is_mouse_button_pressed;
use macroquad::input::mouse_position;

use crate::renderer::SQUARE_SIZE;
use crate::renderer::START_POINT;

pub struct InputPackage {
    pub left_mouse_index: Option<i32>,
}
pub fn gather_input(input_package: &mut InputPackage) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let val = mouse_pos_to_board_index(&mouse_position(), &START_POINT, &SQUARE_SIZE);
        if input_package.left_mouse_index == None || input_package.left_mouse_index != val {
            input_package.left_mouse_index = val;
        }
    };
}
