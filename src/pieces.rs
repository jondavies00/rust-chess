use std::fmt::{Display, Formatter, Result};


#[derive(PartialEq, Clone)]
pub enum Colour {
    White,
    Black,
}
#[derive(Clone)]
pub struct Piece {
    pub name: String,
    symbols: [char; 2],
    pub colour: Colour,
    pub unit_moves: Vec<Vec<i8>>,
    pub capture_moves: Option<Vec<Vec<i8>>>,
    pub multiplier: bool,
    valid_moves: Option<Vec<Vec<u8>>>,
}



pub fn create_pawn(colour: Colour) -> Piece {
    Piece {
        name: String::from("Pawn"),
        symbols: ['♙', '♟'],
        colour: colour,
        //move_set: vec![vec![0, 1], vec![0, 2]],
        unit_moves: vec![vec![0,1], vec![0,2]],
        capture_moves: Some(vec![vec![1, 1], vec![-1, 1]]),
        multiplier: false,
        valid_moves: None,
    }
}

pub fn create_rook(colour: Colour) -> Piece {
    Piece {
        name: String::from("Rook"),
        symbols: ['♖', '♜'],
        colour: colour,
        unit_moves: vec![vec![0,1], vec![0,-1],vec![1,0], vec![-1,0]],
        capture_moves: None,
        multiplier: true,
        valid_moves: None,
    }
}

pub fn create_bishop(colour: Colour) -> Piece {
    Piece {
        name: String::from("Bishop"),
        symbols: ['♗', '♝'],
        colour: colour,
        //move_set: vec![vec![0, 1], vec![0, 2]],
        unit_moves: vec![vec![1,1], vec![1,-1],vec![1,-1], vec![-1,-1]],
        capture_moves: None,
        multiplier: true,
        valid_moves: None,
    }
}

pub fn create_knight(colour: Colour) -> Piece {
    Piece {
        name: String::from("Knight"),
        symbols: ['♘', '♞'],
        colour: colour,
        //move_set: vec![vec![0, 1], vec![0, 2]],
        unit_moves: vec![vec![1,2], vec![1,-2],vec![-1,2], vec![-1,-2], vec![2,1],vec![2,-1],vec![-2,1],vec![-2,-1]],
        capture_moves: Some(vec![vec![1,2], vec![1,-2],vec![-1,2], vec![-1,-2], vec![2,1],vec![2,-1],vec![-2,1],vec![-2,-1]]),
        multiplier: false,
        valid_moves: None,
    }
}

pub fn create_king(colour: Colour) -> Piece {
    Piece {
        name: String::from("King"),
        symbols: ['♔', '♚'],
        colour: colour,
        unit_moves: vec![vec![0, 1], vec![0, -1],vec![1, 0],vec![-1, 0],vec![1, 1],vec![1, -1],vec![-1, 1],vec![-1, -1]],
        capture_moves: None,
        multiplier: false,
        valid_moves: None,
    }
}

pub fn create_queen(colour: Colour) -> Piece {
    Piece {
        name: String::from("Queen"),
        symbols: ['♕', '♛'],
        colour: colour,
        //move_set: vec![vec![0, 1], vec![0, 2]],
        unit_moves: vec![vec![0,1], vec![0,-1],vec![1,0], vec![-1,0], vec![1,1], vec![1,-1],vec![1,-1], vec![-1,-1]],
        capture_moves: None,
        multiplier: true,
        valid_moves: None,
    }
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
            Colour::Black => write!(f, "{}", self.symbols[0]),
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Colour::White => write!(f, "white"),
            Colour::Black => write!(f, "black"),
        }
    }
}
