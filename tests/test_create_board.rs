
use std::num::ParseIntError;

use chess::game::{Move};
use chess::errors::{FormatMoveError};

#[test]
fn test_create_pawn_row() {
    let result = 2+2;
    assert_eq!(result, 4)
}

#[test]
fn test_format_coord() {
    let expected: Move = Move{x:4, y: 4};
    let formatted = Move::from_string_coord(String::from("e4")).unwrap();
    assert_eq!(expected.x, formatted.x);
    assert_eq!(expected.y, formatted.y);
}


#[test]
fn test_format_invalid_coord() {
    // let expected = FormatMoveError;
    let formatted = Move::from_string_coord(String::from("x0"));
    let expected = FormatMoveError;
    assert!(matches!(formatted, Err(expected)));

}