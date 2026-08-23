use macroquad::{color::WHITE, text::draw_text, window::screen_width};

use crate::board::pieces::Sides;

pub fn handle_checkmate(side: Sides) {
    draw_text(
        format!("{side:?} was checkmated"),
        (screen_width() - (20.0 * 14.0)) / 2.0,
        32.0,
        32.0,
        WHITE,
    );
}

pub fn handle_stalemate(side: Sides) {
    draw_text(
        format!("{side:?} was stalemated"),
        (screen_width() - (20.0 * 14.0)) / 2.0,
        32.0,
        32.0,
        WHITE,
    );
}

pub fn handle_draw_by_75_move_rule() {
    draw_text(
        format!("draw by 75 move rule"),
        (screen_width() - (20.0 * 14.0)) / 2.0,
        32.0,
        32.0,
        WHITE,
    );
}

pub fn handle_draw_by_repititon() {
    draw_text(
        format!("draw by repetition"),
        (screen_width() - (20.0 * 14.0)) / 2.0,
        32.0,
        32.0,
        WHITE,
    );
}
