use macroquad::texture::{Texture2D, load_texture};

use crate::piece_textures;

pub struct PieceTextures {
    pub white_king: Texture2D,
    pub black_king: Texture2D,
    pub white_queen: Texture2D,
    pub black_queen: Texture2D,
    pub white_pawn: Texture2D,
    pub black_pawn: Texture2D,
    pub white_rook: Texture2D,
    pub black_rook: Texture2D,
    pub white_bishop: Texture2D,
    pub black_bishop: Texture2D,
    pub white_knight: Texture2D,
    pub black_knight: Texture2D,
}

pub async fn load_all_textures() -> PieceTextures {
    return PieceTextures {
        //first load all the white pieces
        white_king: load_texture("assets/white_king.png")
            .await
            .expect("failed to load white king file"),
        white_bishop: load_texture("assets/white_bishop.png")
            .await
            .expect("failed to load white bishop file"),
        white_knight: load_texture("assets/white_knight.png")
            .await
            .expect("failed to load white knight file"),
        white_pawn: load_texture("assets/white_pawn.png")
            .await
            .expect("failed to load white pawn file"),
        white_queen: load_texture("assets/white_queen.png")
            .await
            .expect("failed to load white queen file"),
        white_rook: load_texture("assets/white_rook.png")
            .await
            .expect("failed to load white rook file"),

        // now all the black piece textures
        black_king: load_texture("assets/black_king.png")
            .await
            .expect("failed to load black king file"),
        black_bishop: load_texture("assets/black_bishop.png")
            .await
            .expect("failed to load black bishop file"),
        black_knight: load_texture("assets/black_knight.png")
            .await
            .expect("failed to load black knight file"),
        black_pawn: load_texture("assets/black_pawn.png")
            .await
            .expect("failed to load black pawn file"),
        black_queen: load_texture("assets/black_queen.png")
            .await
            .expect("failed to load black queen file"),
        black_rook: load_texture("assets/black_rook.png")
            .await
            .expect("failed to load black rook file"),
    };
}
