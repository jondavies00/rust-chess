use std::fmt::{Display, Formatter, Result};



#[derive(PartialEq)]
#[derive(Clone)]
pub enum Colour {
    White,
    Black
}
#[derive(Clone)]
pub struct Piece{
    name: String,
    symbols: [char; 2],
    pub colour: Colour,
    pub move_set: Vec<Vec<u8>>,
    valid_moves: Option<Vec<Vec<u8>>>
}

pub fn create_pawn(colour: Colour) -> Piece {
    Piece { name: String::from("Pawn"), symbols: ['♙', '♟'], colour: colour, move_set: vec![vec![0,1],vec![0,2]], valid_moves: None}
}

pub fn create_rook(colour: Colour) -> Piece {
    Piece { name: String::from("Rook"), symbols: ['♖', '♜'], colour: colour,  move_set: vec![vec![0,1],vec![0,2]], valid_moves: None}
}

pub fn create_bishop(colour: Colour) -> Piece {
    Piece { name: String::from("Bishop"), symbols: ['♗', '♝'], colour: colour, move_set: vec![vec![0,1],vec![0,2]], valid_moves: None}
}

pub fn create_knight(colour: Colour) -> Piece {
    Piece { name: String::from("Knight"), symbols: ['♘', '♞'], colour: colour, move_set: vec![vec![0,1],vec![0,2]], valid_moves: None}
}

pub fn create_king(colour: Colour) -> Piece {
    Piece { name: String::from("King"), symbols: ['♔', '♚'], colour: colour, move_set: vec![vec![0,1],vec![0,2]], valid_moves: None}
}

pub fn create_queen(colour: Colour) -> Piece {
    Piece { name: String::from("Queen"), symbols: ['♕', '♛'], colour: colour, move_set: vec![vec![0,1],vec![0,2]], valid_moves: None}
}




// impl Piece{
//     pub fn create_pawn() -> Piece {
//         Piece { name: String::from("Pawn"), symbol: '♙', move_set: vec![vec![0,1],vec![0,2]]}
//     }
// }

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.colour {
            Colour::White => write!(f, "{}", self.symbols[1]),
            Colour::Black => write!(f, "{}", self.symbols[0])
        }
        
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Colour::White => write!(f, "white"),
            Colour::Black => write!(f, "black")
        }
        
    }
}


