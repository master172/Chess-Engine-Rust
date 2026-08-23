use macroquad::prelude::*;
use std::env;

use crate::{
    ChessState::{DrawByInsufficientMaterial, Regular},
    board::{
        BoardResult, BoardState,
        pieces::Sides::{self},
    },
    draw_manager::DrawDetails,
    endgame_manager::{
        handle_checkmate, handle_draw_by_75_move_rule, handle_draw_by_insufficient_material,
        handle_draw_by_repititon, handle_stalemate,
    },
    fen_engine::fen_to_board_state,
    game::{GameState, MoveResult, handle_game_state, handle_promotion},
    input::{InputPackage, handle_promotion_input},
    piece_textures::{PieceTextures, load_all_textures},
    renderer::{draw_legal_squares, handle_overlays, render_board, render_pieces},
};

mod board;
mod draw_manager;
mod endgame_manager;
mod fen_engine;
mod game;
mod input;
pub mod lookup_helpers;
mod piece_textures;
mod renderer;
mod zorbist_keys;

/// the enum that the main function state machine uses to decide wether to handle input processing for the board
/// or the promoton UI.
pub enum ChessState {
    Regular,
    Promotion(Sides),
    StaleMate(Sides),
    CheckMate(Sides),
    DrawBy75MoveRule,
    DrawByRepition,
    DrawByInsufficientMaterial,
}

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

    let mut prev_result: ChessState = Regular;

    // then preparing the game and board state
    let mut board_state = fen_to_board_state(starting_string);

    let mut game_state: GameState = GameState::new(
        dev_mode,
        board_state.side_to_start,
        board_state.castling_rights,
    );

    board_state.set_attacked_squares(board_state.side_to_start.flip(), &mut game_state);
    board_state.handle_king_saftey(board_state.side_to_start, &mut game_state);
    board_state.gen_all_legal_moves(&mut game_state, board_state.side_to_start);

    //also prepare the draw manager
    let mut draw_details: DrawDetails = DrawDetails::new(&game_state, &board_state);

    if draw_details.insufficient_material(&board_state) {
        prev_result = DrawByInsufficientMaterial
    }

    //loading textures and starting setting up input packages
    let piece_textures: PieceTextures = load_all_textures().await;
    let mut input_package: InputPackage = InputPackage {
        left_mouse_index: None,
    };

    //here is the actual game loop logic
    loop {
        match prev_result {
            ChessState::Regular => {
                //println!("{}", get_fps());
                render_board();
                prev_result = process_input(
                    &mut input_package,
                    &mut game_state,
                    &mut board_state,
                    &mut draw_details,
                );
                draw_legal_squares(&game_state);
                //debug_draw(&game_state);
                render_pieces(&board_state, &piece_textures);
            }
            ChessState::Promotion(side) => {
                render_board();
                render_pieces(&board_state, &piece_textures);
                match handle_promotion_input(side) {
                    None => (),
                    Some(val) => match handle_promotion(&mut game_state, &mut board_state, val) {
                        BoardResult::CheckMate(side) => prev_result = ChessState::CheckMate(side),
                        BoardResult::StaleMate(side) => prev_result = ChessState::StaleMate(side),
                        _ => prev_result = ChessState::Regular,
                    },
                }
            }
            ChessState::StaleMate(side) => {
                render_board();
                render_pieces(&board_state, &piece_textures);
                handle_stalemate(side);
            }
            ChessState::CheckMate(side) => {
                render_board();
                render_pieces(&board_state, &piece_textures);
                handle_checkmate(side);
            }
            ChessState::DrawBy75MoveRule => {
                render_board();
                render_pieces(&board_state, &piece_textures);
                handle_draw_by_75_move_rule();
            }
            ChessState::DrawByRepition => {
                render_board();
                render_pieces(&board_state, &piece_textures);
                handle_draw_by_repititon();
            }
            ChessState::DrawByInsufficientMaterial => {
                render_board();
                render_pieces(&board_state, &piece_textures);
                handle_draw_by_insufficient_material();
            }
        }
        next_frame().await;
    }
}

fn process_input(
    input_package: &mut InputPackage,
    game_state: &mut GameState,
    board_state: &mut BoardState,
    draw_details: &mut DrawDetails,
) -> ChessState {
    let mut result: ChessState = ChessState::Regular;
    match input_package.gather_input() {
        input::States::Idle => (),
        input::States::Update => {
            game_state.input_to_game_state(input_package);
            match handle_game_state(game_state, board_state, draw_details) {
                MoveResult::Move => input_package.reset_input(),
                MoveResult::Promotion(side) => result = ChessState::Promotion(side),
                MoveResult::CheckMate(side) => result = ChessState::CheckMate(side),
                MoveResult::StaleMate(side) => result = ChessState::StaleMate(side),
                MoveResult::DrawByRepition => result = ChessState::DrawByRepition,
                MoveResult::DrawByInsufficientMaterial => {
                    result = ChessState::DrawByInsufficientMaterial
                }
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

    if draw_details.draw_by_excessive_non_progressive_moves() {
        return ChessState::DrawBy75MoveRule;
    }

    result
}
