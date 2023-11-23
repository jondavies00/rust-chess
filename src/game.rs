#[path = "board.rs"] mod board;
#[path = "pieces.rs"] mod pieces;
#[path = "errors.rs"] mod errors;
#[path = "move_piece.rs"] mod move_piece;

use std::{fmt::Error, num::ParseIntError, convert, char::ParseCharError, io};

use board::Board;
use pieces::{Colour};

use move_piece::{Move};


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

     
    pub fn begin(&self)  {
        loop {
            println!("{}", &self.game_config.board.display_string());
            println!("{}", String::from("Please input a move: "));
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    let current_move = Move::from_string_coord(input);
                    match current_move {
                        Ok(current_move) => {
                            println!("{}", String::from(format!("Your move: from {},{} to {}, {}", current_move.x1, current_move.y1,current_move.x2,current_move.x2)));
                            register_move(&self.game_config.board, current_move);
                        }
                        Err(error) => {
                            println!("{}", String::from("Invalid move! Please try again..."));
                            continue;
                        }
                    }
                    
                }
                Err(error) => println!("error: {error}"),
            }
        }

    }

    // pub fn translate_move(coord: String) -> Move {

    // }
}

pub fn register_move(board: &Board, move_: Move) {

    // Find the piece at source  position return error if not
    let source_piece = board.get_piece_at(&move_.x1, &move_.y1);

    match source_piece {
        Some(piece) => {
            println!("{}", String::from("PIECE AT SOURCE COORD"));
            let target_piece = board.get_piece_at(&move_.x2, &move_.y2);
            match target_piece {
                Some(piece) => {
                    println!("{}", String::from("PIECE AT TARGET COORD"));
        
        
                }
                None => {
                    println!("{}", String::from("NO PIECE AT TARGET COORD"));
                }
            }


        }
        None => {
            println!("{}", String::from("NO PIECE AT SOURCE COORD"));
        }
    }

}