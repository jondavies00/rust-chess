#[path = "board.rs"] mod board;
#[path = "pieces.rs"] mod pieces;
#[path = "errors.rs"] mod errors;
use std::{fmt::Error, num::ParseIntError, convert, char::ParseCharError};

use board::{Board};
use pieces::{Colour};

use crate::game::errors::FormatMoveError;


fn create_game() {
    let b = Board::new();
    print!("{}", b.display_string());
}

pub struct GameConfig {
    board: Board,
    turn: Colour
}

pub struct Game {
    game_config: GameConfig
}

pub struct Move {
    pub x: u8,
    pub y: u8
}

pub const FILES: [char;8] = ['a','b','c','d','e','f','g','h'];

impl Move {

    

    pub fn from_string_coord(coord: String) -> Result<Move, errors::FormatMoveError> {
        let file = coord[0..1].parse::<char>();
        let rank = coord[1..2].parse::<u8>();
        

        println!("{}", String::from("Vars"));

        match rank {
            Ok(rank) => {
                println!("Rank: {}", rank);
    
                match file {
                    Ok(file) => {

                        println!("{}", String::from("Vars")); 
                        let file_pos = FILES.iter().position(|r| *r == file);
                        match file_pos {
                            Some(file_pos) => {
                                return Ok(Move {x: file_pos as u8, y: rank});
                            },
                            None => {
                                return Err(FormatMoveError);
                            }
                        }},
                    Err(file) => {println!("{}", String::from("Invalid File!"));return Err(FormatMoveError)}
                }
            
            },
            Err(rank) => {println!("{}", String::from("Invalid rank!")); return Err(FormatMoveError)}
            
        }
    }
}

// Public interface for interacting with a game configuration
impl Game {
    pub fn new(config: Option<GameConfig>) -> Game {
        match config {
            None => Game {game_config: GameConfig {board: Board::new(), turn: Colour::White}},
            Some(x) => Game {game_config: x}
        }
    }

    // pub fn move(&self, coord1: String, coord2: String) -> GameConfig{

    // }

    // pub fn translate_move(coord: String) -> Move {

    // }
}