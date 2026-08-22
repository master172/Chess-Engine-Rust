use chess::mouse_pos_to_board_index;
use macroquad::input::MouseButton;
use macroquad::input::is_mouse_button_pressed;
use macroquad::input::mouse_position;
use macroquad::math::Vec2;
use macroquad::ui::root_ui;
use macroquad::ui::widgets;

use crate::board::BB;
use crate::board::BN;
use crate::board::BQ;
use crate::board::BR;
use crate::board::WB;
use crate::board::WN;
use crate::board::WQ;
use crate::board::WR;
use crate::board::pieces::Sides;
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

const UI_START_POS_X: f32 = 32.0;
const UI_START_POS_Y: f32 = 32.0;
const UI_BUTTON_WIDTH: f32 = 48.0;
const UI_BUTTON_GAP: f32 = 16.0;

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

    pub fn reset_input(&mut self) {
        self.left_mouse_index = None;
    }
}

pub fn handle_promotion_input(side: Sides) -> Option<usize> {
    if root_ui().button(Vec2::new(UI_START_POS_X, UI_START_POS_Y), "Queen") {
        let result: usize = if side == Sides::WHITE { WQ } else { BQ };
        return Some(result);
    }

    if root_ui().button(
        Vec2::new(
            UI_START_POS_X,
            UI_START_POS_Y + (UI_BUTTON_WIDTH) + (UI_BUTTON_GAP),
        ),
        "Rook",
    ) {
        let result: usize = if side == Sides::WHITE { WR } else { BR };
        return Some(result);
    }
    if root_ui().button(
        Vec2::new(
            UI_START_POS_X,
            UI_START_POS_Y + (UI_BUTTON_WIDTH * 2.0) + (UI_BUTTON_GAP * 2.0),
        ),
        "Bishop",
    ) {
        let result: usize = if side == Sides::WHITE { WB } else { BB };
        return Some(result);
    }
    if root_ui().button(
        Vec2::new(
            UI_START_POS_X,
            UI_START_POS_Y + (UI_BUTTON_WIDTH * 3.0) + (UI_BUTTON_GAP * 3.0),
        ),
        "Knight",
    ) {
        let result: usize = if side == Sides::WHITE { WN } else { BN };
        return Some(result);
    }
    return None;
}
