use chess::mouse_pos_to_board_index;
use macroquad::input::MouseButton;
use macroquad::input::is_mouse_button_pressed;
use macroquad::input::mouse_position;

use crate::input::States::Idle;
use crate::input::States::Update;
use crate::renderer::SQUARE_SIZE;
use crate::renderer::START_POINT;

pub struct InputPackage {
    pub left_mouse_index: Option<i32>,
}

pub enum States {
    Idle,
    Update,
}

impl InputPackage {
    pub fn gather_input(&mut self) -> States {
        if is_mouse_button_pressed(MouseButton::Left) {
            let val = mouse_pos_to_board_index(&mouse_position(), &START_POINT, &SQUARE_SIZE);
            if self.left_mouse_index == None || self.left_mouse_index != val {
                self.left_mouse_index = val;
                return Update;
            }
        };
        return Idle;
    }
}
