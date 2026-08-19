use macroquad::prelude::*;

use crate::{
    fen_engine::fen_to_board_state,
    input::{InputPackage, gather_input},
    piece_textures::{PieceTextures, load_all_textures},
    renderer::{handle_overlays, render_board, render_pieces},
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
    let mut input_package: InputPackage = InputPackage {
        left_mouse_index: None,
    };
    loop {
        //println!("{}", get_fps());
        render_board();
        gather_input(&mut input_package);
        handle_overlays(&input_package);
        render_pieces(&board_state, &piece_textures);
        next_frame().await;
    }
}
