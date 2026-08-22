use chess::get_standard_oritentation_index;

use crate::board::{BB, BK, BN, BP, BQ, BR, BoardState, WB, WK, WN, WP, WQ, WR, pieces::Sides};

pub fn fen_to_board_state(input: &str) -> BoardState {
    let fen_parts: Vec<&str> = input.split(" ").collect();
    assert_eq!(
        fen_parts.len(),
        6,
        "the fen string should follow the standard fen notation with six distinct parts"
    );

    let side_to_start = match fen_parts[1].trim() {
        "w" => Sides::WHITE,
        "b" => Sides::BLACK,
        _ => panic!("invalid fen string"),
    };
    let mut board_state: BoardState = BoardState::new(side_to_start);

    let mut square_index: u64 = 0;

    for i in fen_parts[0].chars() {
        match i {
            //first all the black pieces
            'k' => set_piece(&mut board_state, BK, &mut square_index),
            'q' => set_piece(&mut board_state, BQ, &mut square_index),
            'p' => set_piece(&mut board_state, BP, &mut square_index),
            'n' => set_piece(&mut board_state, BN, &mut square_index),
            'b' => set_piece(&mut board_state, BB, &mut square_index),
            'r' => set_piece(&mut board_state, BR, &mut square_index),
            //now all the white pieces
            'K' => set_piece(&mut board_state, WK, &mut square_index),
            'Q' => set_piece(&mut board_state, WQ, &mut square_index),
            'P' => set_piece(&mut board_state, WP, &mut square_index),
            'N' => set_piece(&mut board_state, WN, &mut square_index),
            'B' => set_piece(&mut board_state, WB, &mut square_index),
            'R' => set_piece(&mut board_state, WR, &mut square_index),
            //parse the numbers 1 through 8 since you can never skip less than 1 or more than 8 indexes
            '1'..='8' => {
                square_index +=
                    i.to_digit(10)
                        .expect("this error message should never occur") as u64
            }
            //square_index mod 8 is number of squares consumed in the current rank so subtracting them from 8 should
            // give the number of squares to the next rank
            '/' => {
                if square_index % 8 != 0 {
                    square_index += 8 - (square_index % 8);
                }
            }

            //any other symbol here is invalid
            _ => eprintln!("this is invalid in the notaion, {i}"),
        };
    }

    //for castling rights bit 1 is black queen side, 2 is black king side, 3 is white queen side and 4 is white king side
    match fen_parts[2] {
        //white only
        "K" => board_state.castling_rights = 0b0000_1000,
        "Q" => board_state.castling_rights = 0b0000_0100,
        "KQ" => board_state.castling_rights = 0b0000_1100,

        //black only
        "k" => board_state.castling_rights = 0b0000_0010,
        "q" => board_state.castling_rights = 0b0000_0001,
        "kq" => board_state.castling_rights = 0b0000_0011,

        //mixed this should go K, Q, KQ, k, Kk, Qk, KQk, q, Kq, Qq, KQq, kq, Kkq, Qkq, KQkq - the ones we have done
        "Kk" => board_state.castling_rights = 0b0000_1010,
        "Qk" => board_state.castling_rights = 0b0000_0110,
        "KQk" => board_state.castling_rights = 0b0000_1110,
        "Kq" => board_state.castling_rights = 0b0000_1001,
        "Qq" => board_state.castling_rights = 0b0000_0101,
        "KQq" => board_state.castling_rights = 0b0000_1101,
        "Kkq" => board_state.castling_rights = 0b0000_1011,
        "Qkq" => board_state.castling_rights = 0b0000_0111,
        //all
        "KQkq" => board_state.castling_rights = 0b0000_1111,
        //anything else
        "-" => board_state.castling_rights = 0b0000_0000,
        _ => {
            eprintln!("invalid castlign rights");
            board_state.castling_rights = 0b0000_0000
        }
    }

    board_state
}

fn set_piece(board_state: &mut BoardState, index: usize, square_index: &mut u64) {
    //need to transform the square_index to standard orientation by flipping the ranks
    let board_index: u64 = get_standard_oritentation_index(*square_index as i32) as u64;

    board_state.init_piece(index, board_index);
    *square_index += 1;
}
