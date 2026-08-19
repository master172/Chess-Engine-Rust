use macroquad::input::MouseButton;
use macroquad::input::is_mouse_button_down;
use macroquad::input::mouse_position;

pub fn gather_input() {
    if is_mouse_button_down(MouseButton::Left) {
        println!("mouse button down at pos {:?} ", mouse_position())
    }
}
