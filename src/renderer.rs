use chess::{FileAndRank, Vector2, get_file_and_rank};
use macroquad::{
    color::{BLACK, Color, WHITE},
    math::vec2,
    shapes::draw_rectangle,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
    window::clear_background,
};

use crate::{
    board::{BB, BK, BN, BP, BQ, BR, BoardState, WB, WK, WN, WP, WQ, WR},
    game::GameState,
    input::InputPackage,
    piece_textures::PieceTextures,
};

mod overlay_manager;

pub const START_POINT: Vector2 = Vector2 {
    x: ((1280 / 2) - 64 * 6) as f32,
    y: ((720 / 2) - 64 * 5) as f32,
};
pub const SQUARE_SIZE: Vector2 = Vector2 { x: 64.0, y: 64.0 };

pub const LIGHT_CELL_COLOR: Color = Color::from_hex(0xEEEED2);
pub const DARK_CELL_COLOR: Color = Color::from_hex(0x769656);

//function definitions begin from here
pub fn render_board() {
    clear_background(BLACK);
    draw_board();
}

pub fn render_pieces(board: &BoardState, piece_textures: &PieceTextures) {
    draw_all_pieces(&board, &piece_textures);
}

fn draw_board() {
    let mut current_point: Vector2 = START_POINT;
    for i in 0..64 {
        increment_current_point(i, &mut current_point);
        draw_rectangle(
            current_point.x,
            current_point.y,
            SQUARE_SIZE.x,
            SQUARE_SIZE.y,
            get_current_color(i),
        );
    }
}

fn standard_board_index_to_texture_pos(index: u64) -> Vector2 {
    let board_start_pos_x: f32 = START_POINT.x;
    let board_start_pos_y: f32 = START_POINT.y + (7.0 * SQUARE_SIZE.y);
    let files_and_rank: FileAndRank = get_file_and_rank(index as i32);
    let x_pos: f32 = board_start_pos_x + (SQUARE_SIZE.x * files_and_rank.file as f32);
    let y_pos: f32 = board_start_pos_y - (SQUARE_SIZE.y * files_and_rank.rank as f32);
    Vector2::new(x_pos, y_pos)
}

pub fn draw_texture_at_pos(index: u64, texture: &Texture2D) {
    assert!(
        index <= 63,
        "index must be lesser than 63 to draw the piece"
    );
    let position_to_render: Vector2 = standard_board_index_to_texture_pos(index);
    let draw_params: DrawTextureParams = DrawTextureParams {
        dest_size: Some(vec2(SQUARE_SIZE.x, SQUARE_SIZE.y)),
        ..Default::default()
    };
    draw_texture_ex(
        texture,
        position_to_render.x,
        position_to_render.y,
        WHITE,
        draw_params,
    );
}

fn draw_all_pieces(board_state: &BoardState, textures: &PieceTextures) {
    for (bit_board_index, bit_board) in board_state.board_representation.iter().enumerate() {
        for square_index in 0..64 {
            draw_piece(bit_board, square_index, bit_board_index, textures);
        }
    }
}

fn draw_piece(
    bit_board: &u64,
    square_index: i32,
    bit_board_index: usize,
    textures: &PieceTextures,
) {
    if bit_board & (1 << square_index) == 0 {
        return;
    } else {
        draw_texture_at_pos(
            square_index as u64,
            return_texture_from_array_index(bit_board_index as usize, &textures)
                .expect("failed to load texture"),
        );
    }
}

fn increment_current_point(i: i32, point: &mut Vector2) {
    if i == 0 {
        return;
    }
    if i % 8 == 0 {
        point.x = START_POINT.x;
        point.y += SQUARE_SIZE.y;
    } else {
        point.x += SQUARE_SIZE.x;
    }
}

fn get_current_color(i: i32) -> Color {
    return if get_file_and_rank(i).sum() % 2 == 0 {
        LIGHT_CELL_COLOR
    } else {
        DARK_CELL_COLOR
    };
}

fn return_texture_from_array_index(index: usize, textures: &PieceTextures) -> Option<&Texture2D> {
    match index {
        WK => Some(&textures.white_king),
        BK => Some(&textures.black_king),
        WQ => Some(&textures.white_queen),
        BQ => Some(&textures.black_queen),
        WP => Some(&textures.white_pawn),
        BP => Some(&textures.black_pawn),
        WB => Some(&textures.white_bishop),
        BB => Some(&textures.black_bishop),
        WR => Some(&textures.white_rook),
        BR => Some(&textures.black_rook),
        WN => Some(&textures.white_knight),
        BN => Some(&textures.black_knight),
        _ => None,
    }
}

//overlay code
pub fn handle_overlays(input_package: &InputPackage) {
    match input_package.left_mouse_index {
        Some(val) => draw_overlay_square(val, overlay_manager::SELECTED_PIECE),
        None => (),
    }
}

fn draw_overlay_square(index: i32, color: Color) {
    let pos: Vector2 = standard_board_index_to_texture_pos(index as u64);
    draw_rectangle(pos.x, pos.y, SQUARE_SIZE.x, SQUARE_SIZE.y, color);
}

pub fn draw_legal_squares(game_state: &GameState) {
    if game_state.legal_moves == 0 {
        return;
    };
    let mut mask = game_state.legal_moves;
    while mask != 0 {
        let index = mask.trailing_zeros();
        draw_overlay_square(index as i32, overlay_manager::LEGAL_SQUARE);
        // subtracting 1 turns all 0's before the first 1 to 1 and the first 1 to 0 the and removes all of them
        mask &= mask - 1;
    }
}
