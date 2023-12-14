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
        // Main game loop
        loop {
            println!("{}", &self.game_config.board.display_string());
            println!("{}", String::from(format!("{}, please input a move: ", &self.game_config.turn)));
            
            let mut move_complete = false;
            // Wait for valid move
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
                                        continue;
                                    }
                                }
                                
                            }
                            Err(error) => {
                                println!("{}", error);
                                continue;
                            }
                        }
                        
                    }
                    
                    Err(error) => {println!("{}", String::from("Could not read input.")); println!("{}", error);},
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
                Some(target_piece) => {

                    if (&target_piece.colour == turn_colour){
                        println!("{}", String::from("Piece of same colour at target"));
                        return Err(InvalidMoveError);
                    }

                    if validate_move(&move_, board, piece, true) {
                        println!("{}", String::from("Capturing piece"));
                        board.update_captured(target_piece.clone());
                        board.move_piece_to(&move_.x1, &move_.y1, &move_.x2, &move_.y2);
                        return Ok(());
                    }
                }
                None => {
                    if validate_move(&move_, board, piece, false) {
                        println!("{}", String::from("NO PIECE AT TARGET COORD"));

                        println!("{}", String::from("Moving piece!"));
                        board.move_piece_to(&move_.x1, &move_.y1, &move_.x2, &move_.y2);
                        return Ok(());
                    }

                    
                    
                }
            }

            return Err(InvalidMoveError)
            
            


        }
        None => {
            println!("{}", String::from("NO PIECE AT SOURCE COORD"));
            return Err(InvalidMoveError);
        }
    }
    
}

pub fn get_unit_move(new_move: &Vec<i8>, unit_moves: &Vec<Vec<i8>>) -> Option< Vec<i8>>  {

    for unit_move in unit_moves {
    println!("{}", String::from(format!("unit move is {},{}. new move is {}, {}", unit_move[0], unit_move[1], new_move[0], new_move[1])));
        if new_move[0] * unit_move[0] > 0 && new_move[1] * unit_move[1] > 0 {
            return Some(unit_move.clone());
        }
        else if  new_move[0] * unit_move[0] == 0  && new_move[0] == 0 && unit_move[0] == 0 || new_move[1] * unit_move[1] == 0 && new_move[1] == 0 && unit_move[1] == 0 {
            return Some(unit_move.clone());
        }
    }
    None

}

pub fn validate_move(move_: &Move, board: &Board, source_piece: &Piece, capture: bool) -> bool {
    let x_distance = move_.x2 as i8 - move_.x1 as i8;
    let y_distance = move_.y2 as i8 - move_.y1 as i8;

    let mut new_move = vec!(x_distance, y_distance);

    // Only piece that can't go backwards.
    // Probably a better way to handle
    if source_piece.name == "Pawn" && source_piece.colour == Colour::Black {
        println!("{}", String::from("inversing move"));
        new_move[0] *= -1;
        new_move[1] *= -1;
    }
    let moves_to_check: &Vec<Vec<i8>>;
    match capture {
        true =>  {
            match &source_piece.capture_moves {
                Some(moves) => {
                    moves_to_check = moves;
                }
                None =>
                {
                    moves_to_check = &source_piece.unit_moves;
                }
            }
        }
        false => {
            
            moves_to_check = &source_piece.unit_moves;
        }
    }

    if moves_to_check.contains(&new_move) {
        return true;

    }
    if source_piece.multiplier {
        println!("{}", String::from(format!("Multiplier exists. Checking unit moves")));
        match(get_unit_move(&new_move, moves_to_check)) {
            Some(unit_move) => {
                return is_blocking_piece(move_, &unit_move, board);
            }
            None => {return false;}

        }

    }
    println!("{}", String::from(format!("No multipler and unit moves do not contain new move {},{}", new_move[0], new_move[1])));
    return false;
    



}

// pub fn valid_capture(move_: &Move, board: &Board, source_piece: &Piece, capture: bool) -> bool{
//     let x_distance = move_.x2 as i8 - move_.x1 as i8;
//     let y_distance = move_.y2 as i8 - move_.y1 as i8;

//     let mut new_move = vec!(x_distance, y_distance);

//     // Only piece that can't go backwards.
//     // Probably a better way to handle
//     if source_piece.name == "Pawn" && source_piece.colour == Colour::Black {
//         new_move[0] *= -1;
//         new_move[1] *= -1;
//     }

//     match &source_piece.capture_moves {
//         Some(capture_moves) => {
//             if capture_moves.contains(&new_move) {
//                 return true;
        
//             }
//         }
//         None => {
//             if source_piece.unit_moves.contains(&new_move) {
//                 return true;
        
//             }
//         }

//     }
    
//     if source_piece.multiplier {
//         match(get_unit_move(&new_move, &source_piece.unit_moves)) {
//             Some(unit_move) => {
//                 return is_blocking_piece(move_, &unit_move, board);
//             }
//             None => {return false;}

//         }

//     }
//     println!("{}", String::from(format!("No multipler and unit moves do not contain new move {},{}", new_move[0], new_move[1])));
//     return false;
// }
    

pub fn is_blocking_piece(move_: &Move, unit_move: &Vec<i8>, board: &Board) -> bool {
    // Use the board to find if there's a piece between the piece's source move and target square
    // Generate all squares the piece needs to pass through
    let x_to = move_.x2 as i8;
    let y_to = move_.y2 as i8;
    let mut x_from = move_.x1 as i8;
    let mut y_from = move_.y1 as i8;
    //let start_square = vec![move_.x1, move_.y1];
    println!("{}", String::from(format!("From x: {}, From y: {} To x: {}, To y: {}", x_from, y_from, x_to, y_to)));
    while x_from != x_to + unit_move[0] * -1  && y_from != y_to + unit_move[1] * -1 {
        x_from += unit_move[0];
        y_from += unit_move[1];
        println!("{}", String::from(format!("Looking at {},{}", x_from, y_from)));
        match &board.positions[y_from as usize][x_from as usize] {
            Some(piece) => {
                println!("{}", String::from("BLOCKING PIECE!"));
                return false
            }
            None => {
                continue;
            }
        }
    }
    return true;
}

pub fn move_in_move_set(move_: &Move, move_set: &Vec<Vec<u8>>) -> bool{
    println!("{}", String::from(format!("move x1: {}, x2: {} y1: {}, y2: {}", move_.x1, move_.x2, move_.y1, move_.y2)));
    // TODO: make moveset a range e.g. [0,2] means any x within 0 and y within 2
    let x_distance = move_.x2 as i8 - move_.x1 as i8;
    let y_distance = move_.y2 as i8 - move_.y1 as i8;
    println!("{}", String::from(format!("x dist: {}, y_dist: {}", x_distance, y_distance)));
    let new_move = vec!(x_distance.abs() as u8, y_distance.abs() as u8);
    return move_set.contains(&new_move);
    
}