
use std::num::ParseIntError;

use chess::board::Board;
use chess::game::{Game};
use chess::errors::{FormatMoveError, InvalidMoveError};
use chess::move_piece::Move;
use chess::pieces::Colour;

fn create_from_str_coord_array(str_moves: Vec<&str>) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    for m in str_moves {
        let move_ = Move::from_string_coord(String::from(m));
        match (move_) {
            Ok(move_) => {
                moves.push(move_);
            }
            Err(error) => {
                error;
            }
        }
    }
    return moves;

}

#[test]
fn test_castle() {
    let game:  &mut Game =  &mut Game::new(None);
    let castle_moves: Vec<Move> = create_from_str_coord_array(vec!["e2 e4", "d7 d6", "f1 e2", "d6 d5", "g1 h3", "d5 d4", "e1 h1"]);
    for m in castle_moves {
        let res = game.register_move(m);
        match (res) {
            Ok(res) => {}
            Err(error) => {
                error;
            }
        }
    }

    print!("{}", game.get_board());
    assert!(!game.can_castle(Colour::White));
    assert!(game.can_castle(Colour::Black));
}

#[test]
fn test_cannot_castle() {
    let game:  &mut Game =  &mut Game::new(None);
    let castle_moves: Vec<Move> = create_from_str_coord_array(vec!["e2 e4", "d7 d6", "a2 a4", "d6 d5", "g1 h3", "d5 d4","e1 h1"]);
    for m in castle_moves {
        let res = game.register_move(m);
        match (res) {
            Ok(res) => {}
            Err(error) => {
                error;
            }
        }
    }

    print!("{}", game.get_board());
    assert!(game.can_castle(Colour::White));
    assert!(game.can_castle(Colour::Black));
}

