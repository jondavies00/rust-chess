use std::{fmt::Error, num::ParseIntError, convert, char::ParseCharError, io};

use crate::errors::{InvalidMoveError, self};

use crate::board::Board;
use crate::pieces::{Colour, Piece};

use crate::move_piece::{Move};






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




// Public interface for interacting with a game configuration
impl Game {
    pub fn new(config: Option<GameConfig>) -> Game {
        match config {
            None => Game {game_config: GameConfig {board: Board::new(), turn: Colour::White}},
            Some(x) => Game {game_config: x}
        }
    }

     
    pub fn begin(&mut self)  {
        loop {
            println!("{}", &self.game_config.board.display_string());
            println!("{}", String::from("Please input a move: "));
            
            let mut move_complete = false;
            while move_complete == false {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(n) => {
                        let current_move = Move::from_string_coord(input);
                        match current_move {
                            Ok(current_move) => {
                                println!("{}", String::from(format!("Your move: from {},{} to {}, {}", current_move.x1, current_move.y1,current_move.x2,current_move.x2)));
                                match (make_move(&mut self.game_config.board, &current_move, &self.game_config.turn)) {
                                    Ok(()) => {
                                        move_complete = true;

                                    }
                                    Err(error) => {
                                        println!("{}", error);
                                        move_complete = true;
                                    }
                                }
                                
                            }
                            Err(error) => {
                                println!("{}", error);
                                continue;
                            }
                        }
                        
                    }
                    Err(error) => println!("{}", error),
                }
                
            }
            
            match self.game_config.turn {
                Colour::White => {
                    self.game_config.turn = Colour::Black
                }
                Colour::Black => {
                    self.game_config.turn = Colour::White
                }
            }
            
        }

    }
}


pub fn make_move( board: &mut Board, move_: &Move, turn_colour: &Colour) -> Result<(), InvalidMoveError> {
    // Find the piece at source  position return error if not
    let source_piece = board.get_piece_at(&move_.x1, &move_.y1);

    match source_piece {
        Some(piece) => {
            if (&piece.colour != turn_colour) {
                println!("{}", String::from("Source piece is not of correct colour."));
                println!("{}", String::from(format!("{} != {}", &piece.colour, turn_colour)));
                return Err(InvalidMoveError);
            }
            println!("{}", String::from("PIECE AT SOURCE COORD"));
            let target_piece = board.get_piece_at(&move_.x2, &move_.y2);
            
            match target_piece {
                Some(piece) => {
                    println!("{}", String::from("PIECE AT TARGET COORD"));
                    return Err(InvalidMoveError);
                }
                None => {
                    println!("{}", String::from("NO PIECE AT TARGET COORD"));
                    if (validate_move_for_piece(&move_, &piece.move_set)) {
                        println!("{}", String::from("Moving piece!"));
                        board.move_piece_to(&move_.x1, &move_.y1, &move_.x2, &move_.y2);
                        return Ok(());
                    }
                    else {
                        println!("{}", String::from("Not valid move"));
                        return Err(InvalidMoveError);
                    }
                    
                    
                }
            }


        }
        None => {
            println!("{}", String::from("NO PIECE AT SOURCE COORD"));
            return Err(InvalidMoveError);
        }
    }
    
}

pub fn validate_move_for_piece(move_: &Move, move_set: &Vec<Vec<u8>>) -> bool{
    println!("{}", String::from(format!("move x1: {}, x2: {} y1: {}, y2: {}", move_.x1, move_.x2, move_.y1, move_.y2)));
    // TODO: make moveset a range e.g. [0,2] means any x within 0 and y within 2
    let x_distance = move_.x2 as i8 - move_.x1 as i8;
    let y_distance = move_.y2 as i8 - move_.y1 as i8;
    println!("{}", String::from(format!("x dist: {}, y_dist: {}", x_distance, y_distance)));
    let new_move = vec!(x_distance.abs() as u8, y_distance.abs() as u8);
    return move_set.contains(&new_move);
    
}