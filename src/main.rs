use macroquad::prelude::*;

use crate::{input::gather_input, renderer::render};

mod board;
mod input;
mod renderer;

fn game_conf() -> Conf {
    Conf {
        window_title: String::from("Chess game"),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(game_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_board();
        gather_input();
        next_frame().await
    }
}

fn draw_board() {
    render();
}
