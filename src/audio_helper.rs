use macroquad::audio::{self, Sound};

pub struct AudioPackage {
    pub game_start: Sound,
    pub move_piece_white: Sound,
    pub move_piece_black: Sound,
    pub capture_piece_white: Sound,
    pub capture_piece_black: Sound,
    pub castle_white: Sound,
    pub castle_black: Sound,
    pub check_white: Sound,
    pub check_black: Sound,
    pub game_end: Sound,
    pub checkmate: Sound,
    pub stalemate: Sound,
}

impl AudioPackage {
    pub async fn load_all_sounds() -> Self {
        Self {
            game_start: audio::load_sound("assets/SFX/GameStart.wav").await.unwrap(),
            move_piece_white: audio::load_sound("assets/SFX/Move.wav").await.unwrap(),
            move_piece_black: audio::load_sound("assets/SFX/Move1.wav").await.unwrap(),
            capture_piece_white: audio::load_sound("assets/SFX/Capture.wav").await.unwrap(),
            capture_piece_black: audio::load_sound("assets/SFX/Capture1.wav").await.unwrap(),
            castle_white: audio::load_sound("assets/SFX/Castle.wav").await.unwrap(),
            castle_black: audio::load_sound("assets/SFX/Castle1.wav").await.unwrap(),
            check_white: audio::load_sound("assets/SFX/Check.wav").await.unwrap(),
            check_black: audio::load_sound("assets/SFX/Check1.wav").await.unwrap(),
            game_end: audio::load_sound("assets/SFX/GameEnd.wav").await.unwrap(),
            checkmate: audio::load_sound("assets/SFX/Checkmate.wav").await.unwrap(),
            stalemate: audio::load_sound("assets/SFX/Stalemate.wav").await.unwrap(),
        }
    }
}
