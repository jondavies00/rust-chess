
use std::num::ParseIntError;

use chess::board::Board;
use chess::game::{Game};
use chess::errors::{FormatMoveError};
use chess::move_piece::Move;

// #[test]
// fn test_create_pawn_row() {
//     let board = Board::new();
//     let 
//     println!("{}", Board::gen_home_row(Colour::Black).clone());
//     let first_rank = board.
// }d

#[test]
fn test_game() {
    let game:  &mut Game =  &mut Game::new(None);
    game.begin();}


// #[test]
// fn test_format_coord() {
//     let expected: Move = Move{x:4, y: 4};
//     let formatted = Move::from_string_coord(String::from("e4")).unwrap();
//     assert_eq!(expected.x, formatted.x);
//     assert_eq!(expected.y, formatted.y);
// }


#[test]
fn test_format_invalid_coord() {
    // let expected = FormatMoveError;
    let formatted = Move::from_string_coord(String::from("x0"));
    let expected = FormatMoveError;
    assert!(matches!(formatted, Err(expected)));

}