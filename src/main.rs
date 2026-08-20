use std::env;

use macroquad::prelude::*;

use crate::{
    board::BoardState,
    fen_engine::fen_to_board_state,
    game::{GameState, MoveResult, handle_game_state},
    input::InputPackage,
    piece_textures::{PieceTextures, load_all_textures},
    renderer::{draw_legal_squares, handle_overlays, render_board, render_pieces},
};

mod board;
mod fen_engine;
mod game;
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
    let mut starting_string: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut dev_mode: bool = false;
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        starting_string = &args[1];
    }
    if args.len() > 2 {
        dev_mode = args[2].parse::<bool>().unwrap_or(false);
    }
    let mut board_state = fen_to_board_state(starting_string);

    let mut game_state: GameState = GameState::new(dev_mode, board_state.side_to_start);

    let piece_textures: PieceTextures = load_all_textures().await;
    let mut input_package: InputPackage = InputPackage {
        left_mouse_index: None,
    };
    loop {
        //println!("{}", get_fps());
        render_board();
        process_input(&mut input_package, &mut game_state, &mut board_state);
        draw_legal_squares(&game_state);
        render_pieces(&board_state, &piece_textures);
        next_frame().await;
    }
}

fn process_input(
    input_package: &mut InputPackage,
    game_state: &mut GameState,
    board_state: &mut BoardState,
) {
    match input_package.gather_input() {
        input::States::Idle => (),
        input::States::Update => {
            game_state.input_to_game_state(input_package);
            match handle_game_state(game_state, board_state) {
                MoveResult::Move => input_package.reset_input(),
                _ => (),
            }
        }
    };
    match game_state.current_index {
        Some(val) => {
            if board_state.validate_piece_selection(val as u64)
                && board_state.valid_piece_selection(val, game_state)
            {
                handle_overlays(input_package);
            } else {
                board_state.reset_necessary_game_state_variables(game_state);
                input_package.reset_input();
            }
        }
        None => (),
    }
}
