use std::env;

use macroquad::prelude::*;

use crate::{
    board::{BoardState, pieces::Sides::WHITE},
    fen_engine::fen_to_board_state,
    game::{GameState, MoveResult, handle_game_state},
    input::InputPackage,
    piece_textures::{PieceTextures, load_all_textures},
    renderer::{
        draw_legal_squares, draw_squares_from_num, handle_overlays, render_board, render_pieces,
    },
};

mod board;
mod fen_engine;
mod game;
mod input;
pub mod lookup_helpers;
mod piece_textures;
mod renderer;

fn game_conf() -> Conf {
    Conf {
        window_title: String::from("Chess game"),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        window_resizable: false,

        //platform: miniquad::conf::Platform {
        //    swap_interval: Some(0),
        //    ..Default::default()
        //},
        ..Default::default()
    }
}

//the only reason this code exsits is to help pregenerate some lookup tables

//fn balck_pawn_helpers() {
//    let mut data: String = String::from("pub const BLACK_PAWN_ATTACK_REFERENCE: [u64;64] = [ ");
//    for i in 0..64 {
//        data.push_str(&format!(
//            "{},",
//            Pawn::get_attacking_squares(i, &board::pieces::Sides::WHITE)
//        ));
//    }
//    data.push_str("];");
//    fs::write("output.txt", data).unwrap();
//}

#[macroquad::main(game_conf)]
async fn main() {
    //balck_pawn_helpers();
    // all of this is just setup
    // first the env variables processing
    let mut starting_string: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut dev_mode: bool = false;
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        starting_string = &args[1];
    }
    if args.len() > 2 {
        dev_mode = args[2].parse::<bool>().unwrap_or(false);
    }

    // then preparing the game and board state
    let mut board_state = fen_to_board_state(starting_string);

    let mut game_state: GameState = GameState::new(dev_mode, board_state.side_to_start);

    board_state.set_attacked_squares(board_state.side_to_start.flip(), &mut game_state);
    board_state.handle_king_saftey(board_state.side_to_start, &mut game_state);
    //loading textures and starting setting up input packages
    let piece_textures: PieceTextures = load_all_textures().await;
    let mut input_package: InputPackage = InputPackage {
        left_mouse_index: None,
    };

    //here is the actual game loop logic
    loop {
        //println!("{}", get_fps());
        render_board();
        process_input(&mut input_package, &mut game_state, &mut board_state);
        draw_legal_squares(&game_state);
        //debug_draw(&game_state);
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

#[allow(dead_code)]
#[cfg(debug_assertions)]
fn debug_draw(game_state: &GameState) {
    let current_attacking_squares: &u64 = if game_state.current_side == WHITE {
        &game_state.black_attacked
    } else {
        &game_state.white_attacked
    };

    draw_squares_from_num(*current_attacking_squares, RED);
}
