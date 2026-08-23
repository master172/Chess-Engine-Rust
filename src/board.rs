use crate::{
    GameState,
    board::{
        BoardResult::{CheckMate, StaleMate},
        move_generator::king::King,
        piece_definitions::{BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK},
        pieces::{
            Piece,
            Sides::{self, BLACK, WHITE},
        },
    },
};

pub mod move_generator;
mod piece_definitions;
pub mod pieces;

///index of white king
pub const WK: usize = 0;

///index of black king
pub const BK: usize = 1;

///index of white queen
pub const WQ: usize = 2;

///index of black queen
pub const BQ: usize = 3;

///index of white pawns
pub const WP: usize = 4;

///index of black pawns
pub const BP: usize = 5;

///index of white knights
pub const WN: usize = 6;

///index of black knights
pub const BN: usize = 7;

///index of white rooks
pub const WR: usize = 8;

///index of black rooks
pub const BR: usize = 9;

///index of white bishops
pub const WB: usize = 10;

///index of black bishops
pub const BB: usize = 11;

pub const BLACK_INDEXES: [usize; 6] = [BK, BQ, BN, BR, BB, BP];
pub const WHITE_INDEXES: [usize; 6] = [WK, WQ, WN, WR, WB, WP];

const BLACK_CASTLE_INVALIDATION_MASK: u64 = (1 << 63) | (1 << 56) | (1 << 60);
const WHITE_CASTLE_INVALIDATION_MASK: u64 = (1 << 0) | (1 << 4) | (1 << 7);

const WHITE_PROMOTION_MASK: u64 = 0xff00_0000_0000_0000;
const BLACK_PROMOTION_MASK: u64 = 0xff;

pub enum BoardResult {
    None,
    Capture,
    PawnMove,
    Promotion(Sides),
    CheckMate(Sides),
    StaleMate(Sides),
}

#[derive(Debug)]
pub struct BoardState {
    pub board_representation: [u64; 12],
    pub side_to_start: Sides,
    pub castling_rights: u8,
    pub initial_half_moves: u32,
    pub initial_en_passant_index: usize,
}

impl BoardState {
    pub fn new(side: Sides) -> Self {
        Self {
            board_representation: [0; 12],
            side_to_start: side,
            castling_rights: 0,
            initial_half_moves: 0,
            initial_en_passant_index: 0,
        }
    }

    pub fn init_piece(&mut self, index: usize, square_index: u64) {
        self.board_representation[index] = self.board_representation[index] | 1 << square_index;
    }

    pub fn get_piece_from_index(&self, index: i32) -> Option<(Piece, Sides, usize)> {
        let current_bit_mask: u64 = 1 << index;
        for (index, val) in self.board_representation.iter().enumerate() {
            if val & current_bit_mask != 0 {
                return index_to_piece(index);
            }
        }
        return None;
    }

    pub fn generate_legal_moves(&self, game_state: &mut GameState) {
        if !self.validate_piece_selection(
            game_state
                .current_index
                .expect("again this error message should also never show up") as u64,
        ) {
            return;
        }
        game_state.previous_index = Some(game_state.current_index.unwrap());
        let (piece, side, index) = self
            .get_piece_from_index(game_state.current_index.unwrap())
            .expect("this is also one of those errors that should never show up");
        if game_state.dev_mode == false && side != game_state.current_side {
            return;
        }
        game_state.current_array_index = Some(index);

        piece.pre_move_gen(
            game_state.current_index.unwrap() as u64,
            &self.board_representation,
            &side,
            game_state,
        );
        game_state.legal_moves =
            game_state.all_legal_moves[game_state.current_index.unwrap() as usize];
    }

    pub fn validate_piece_selection(&self, index: u64) -> bool {
        for i in self.board_representation {
            if i & (1 << index) != 0 {
                return true;
            }
        }
        return false;
    }

    pub fn reset_necessary_game_state_variables(&self, game_state: &mut GameState) {
        game_state.legal_moves = 0;
        game_state.previous_index = None;
        game_state.current_index = None;
        game_state.current_array_index = None;
        //game_state.en_passant_candidate_mask = 0;
    }

    pub fn valid_piece_selection(&self, index: i32, game_state: &GameState) -> bool {
        let (_, side, _) = self.get_piece_from_index(index).unwrap();
        return side == game_state.current_side;
    }

    pub fn handle_move_after_effects(
        &mut self,
        game_state: &mut GameState,
        side: Sides,
        piece: Piece,
        prev_index: u64,
        current_index: u64,
    ) {
        match piece {
            PAWN => {
                if game_state.en_passant_capture_mask & (1 << current_index) != 0 {
                    game_state.en_passant_mask = game_state.en_passant_candidate_mask;
                    game_state.en_passant_candidate_mask = 0;
                } else if game_state.en_passant_mask & (1 << current_index) != 0 {
                    self.board_representation[BP] &= !game_state.en_passant_capture_mask;
                    self.board_representation[WP] &= !game_state.en_passant_capture_mask;
                    game_state.en_passant_mask = 0;
                    game_state.en_passant_candidate_mask = 0;
                    game_state.en_passant_capture_mask = 0;
                } else {
                    game_state.en_passant_candidate_mask = 0;
                    game_state.en_passant_capture_mask = 0;
                    game_state.en_passant_mask = 0;
                }
            }
            KING => {
                let rook_index: usize = if side == Sides::WHITE { WR } else { BR };
                match king_movment_to_rook_transposition(
                    current_index as usize,
                    game_state.castling_rights,
                ) {
                    None => (),
                    Some((to, from)) => {
                        self.board_representation[rook_index] &= !from;
                        self.board_representation[rook_index] |= to;
                    }
                }
            }
            _ => {
                // cleanup in case a piece with no special moves associated with it moes
                // they are reset to prevent state corruption
                game_state.en_passant_capture_mask = 0;
                game_state.en_passant_candidate_mask = 0;
                game_state.en_passant_mask = 0;
            }
        }

        if (1 << prev_index) & (BLACK_CASTLE_INVALIDATION_MASK | WHITE_CASTLE_INVALIDATION_MASK)
            != 0
        {
            game_state.castling_rights &= !map_indexes_to_right_to_remove(prev_index as usize);
        }
        if (1 << current_index) & (BLACK_CASTLE_INVALIDATION_MASK | WHITE_CASTLE_INVALIDATION_MASK)
            != 0
        {
            game_state.castling_rights &= !map_indexes_to_right_to_remove(current_index as usize);
        }
    }

    pub fn gen_all_legal_moves(&self, game_state: &mut GameState, side: Sides) {
        let indexes_to_check = if side == Sides::WHITE {
            WHITE_INDEXES
        } else {
            BLACK_INDEXES
        };

        game_state.all_legal_moves = [0; 64];
        game_state.current_legal_move_mask = 0;
        for board_index in indexes_to_check {
            let mut board: u64 = self.board_representation[board_index];
            while board != 0 {
                let index = board.trailing_zeros();
                let (piece, _, _) = index_to_piece(board_index).unwrap();
                let legal_moves: u64 = piece.generate_moves(
                    index as u64,
                    &self.board_representation,
                    &side,
                    game_state,
                );
                game_state.all_legal_moves[index as usize] = legal_moves;
                game_state.current_legal_move_mask |= legal_moves;
                board &= board - 1;
            }
        }
    }

    pub fn check_endgame(&self, side: Sides, game_state: &GameState) -> BoardResult {
        let checks_for_side = if side == Sides::WHITE {
            game_state.white_checks
        } else {
            game_state.black_checks
        };
        if game_state.current_legal_move_mask == 0 && checks_for_side == 0 {
            return BoardResult::StaleMate(side);
        } else if game_state.current_legal_move_mask == 0 && checks_for_side > 0 {
            return BoardResult::CheckMate(side);
        }
        return BoardResult::None;
    }

    pub fn move_piece(&mut self, game_state: &mut GameState) -> BoardResult {
        let mut result: BoardResult = BoardResult::None;
        let (piece, side, _) = self
            .get_piece_from_index(game_state.previous_index.unwrap())
            .unwrap();
        self.board_representation[game_state.current_array_index.unwrap()] &=
            !(1 << (game_state.previous_index.unwrap() as u64));
        self.board_representation[game_state.current_array_index.unwrap()] |=
            1 << (game_state.current_index.unwrap() as u64);
        let capture_mask: u64 = !(1 << (game_state.current_index.unwrap() as u64));

        let relevant_promotion_mask: &mut u64;
        let validation_promotion_mask: u64;
        match side {
            Sides::WHITE => {
                for i in BLACK_INDEXES {
                    if self.board_representation[i] & !capture_mask != 0 {
                        result = BoardResult::Capture
                    }
                    self.board_representation[i] &= capture_mask;
                }
                relevant_promotion_mask = &mut game_state.white_promotion_mask;
                validation_promotion_mask = WHITE_PROMOTION_MASK;
            }
            Sides::BLACK => {
                for i in WHITE_INDEXES {
                    if self.board_representation[i] & !capture_mask != 0 {
                        result = BoardResult::Capture
                    }
                    self.board_representation[i] &= capture_mask;
                }
                relevant_promotion_mask = &mut game_state.black_promotion_mask;
                validation_promotion_mask = BLACK_PROMOTION_MASK;
            }
        }

        //pawn promotion specific
        if piece == PAWN
            && (1 << game_state.current_index.unwrap() as u64) & validation_promotion_mask != 0
        {
            *relevant_promotion_mask |= 1 << game_state.current_index.unwrap() as u64;
            result = BoardResult::Promotion(side);
        } else if piece == PAWN {
            result = BoardResult::PawnMove
        }

        self.set_attacked_squares(side, game_state);
        self.handle_move_after_effects(
            game_state,
            side,
            piece,
            game_state.previous_index.unwrap() as u64,
            game_state.current_index.unwrap() as u64,
        );
        self.handle_king_saftey(side.flip(), game_state);
        self.gen_all_legal_moves(game_state, side.flip());

        self.reset_necessary_game_state_variables(game_state);

        match self.check_endgame(side.flip(), game_state) {
            CheckMate(side) => result = CheckMate(side),
            StaleMate(side) => result = StaleMate(side),
            _ => (),
        }
        if game_state.dev_mode == false {
            game_state.current_side = game_state.current_side.flip();
        }

        result
    }

    // the side passed to it must be the side that is playing currently
    pub fn handle_king_saftey(&self, side: Sides, game_state: &mut GameState) {
        let king_index: usize = if side == Sides::WHITE { WK } else { BK };
        let mut king_mask: u64 = self.board_representation[king_index];
        while king_mask != 0 {
            let index = king_mask.trailing_zeros();
            King::king_saftey(index as u64, side, game_state, &self.board_representation);
            king_mask &= king_mask - 1
        }
    }

    // the side provided to this function must be the other side i.e the side that is not moving now
    pub fn set_attacked_squares(&self, side: Sides, game_state: &mut GameState) {
        let current_indexes: [usize; 6] = if side == Sides::WHITE {
            WHITE_INDEXES
        } else {
            BLACK_INDEXES
        };
        let current_attacks: &mut u64 = if side == Sides::WHITE {
            &mut game_state.white_attacked
        } else {
            &mut game_state.black_attacked
        };
        *current_attacks = 0;
        for board_index in current_indexes {
            let mut mask: u64 = self.board_representation[board_index];
            while mask != 0 {
                let index = mask.trailing_zeros();
                let (piece, side, _) = index_to_piece(board_index).unwrap();

                *current_attacks |=
                    piece.get_attacking_squares(index as u64, &self.board_representation, &side);
                mask &= mask - 1;
            }
        }
    }
}

pub fn index_to_piece(index: usize) -> Option<(Piece, Sides, usize)> {
    match index {
        WK => Some((KING, WHITE, index)),
        BK => Some((KING, BLACK, index)),
        WQ => Some((QUEEN, WHITE, index)),
        BQ => Some((QUEEN, BLACK, index)),
        WP => Some((PAWN, WHITE, index)),
        BP => Some((PAWN, BLACK, index)),
        WB => Some((BISHOP, WHITE, index)),
        BB => Some((BISHOP, BLACK, index)),
        WR => Some((ROOK, WHITE, index)),
        BR => Some((ROOK, BLACK, index)),
        WN => Some((KNIGHT, WHITE, index)),
        BN => Some((KNIGHT, BLACK, index)),
        _ => None,
    }
}

fn map_indexes_to_right_to_remove(index: usize) -> u8 {
    match index {
        0 => 0b0000_0100,
        4 => 0b0000_1100,
        7 => 0b0000_1000,
        56 => 0b0000_0001,
        60 => 0b0000_0011,
        63 => 0b0000_0010,
        _ => 0,
    }
}

fn king_movment_to_rook_transposition(index: usize, castling_rights: u8) -> Option<(u64, u64)> {
    if destination_square_to_validation_int(index) & castling_rights == 0 {
        return None;
    }
    destination_square_to_rook_transposition(index)
}

fn destination_square_to_validation_int(index: usize) -> u8 {
    match index {
        2 => 0b0000_0100,
        6 => 0b0000_1000,
        58 => 0b0000_0001,
        62 => 0b0000_0010,
        _ => 0,
    }
}

//the first resultant int is the place the rook should move to and the second is where it should be removed from
fn destination_square_to_rook_transposition(index: usize) -> Option<(u64, u64)> {
    match index {
        2 => Some((0x8, 0x1)),
        6 => Some((0x20, 0x80)),
        58 => Some((0x800000000000000, 0x100000000000000)),
        62 => Some((0x2000000000000000, 0x8000000000000000)),
        _ => None,
    }
}
