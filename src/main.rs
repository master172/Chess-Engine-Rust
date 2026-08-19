use macroquad::prelude::*;

use crate::{fen_engine::fen_to_baord_state, input::gather_input, renderer::render};

mod board;
mod fen_engine;
mod input;
pub mod renderer;

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
    let current_board_state =
        fen_to_baord_state("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{current_board_state:#?}");
    loop {
        draw_board();
        gather_input();
        next_frame().await
    }
}

fn draw_board() {
    render();
}
