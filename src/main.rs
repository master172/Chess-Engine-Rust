use macroquad::prelude::*;

use crate::{
    board::BoardState,
    fen_engine::fen_to_board_state,
    input::gather_input,
    piece_textures::{PieceTextures, load_all_textures},
    renderer::render,
};

mod board;
mod fen_engine;
mod input;
mod piece_textures;
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
    let board_state =
        fen_to_board_state("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    let piece_textures: PieceTextures = load_all_textures().await;

    loop {
        //println!("{}", get_fps());
        draw_board(&board_state, &piece_textures);
        gather_input();
        next_frame().await
    }
}

fn draw_board(board_state: &BoardState, piece_textures: &PieceTextures) {
    render(&board_state, &piece_textures);
}
