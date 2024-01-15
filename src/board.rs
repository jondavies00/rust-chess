use std::fmt::{Display, Formatter, Result};


use crate::pieces::{Piece, Colour, create_pawn, create_bishop, create_king, create_knight, create_rook, create_queen};

// pub enum Position {
//     Piece,
//     None
// }

type Square = Option<Piece>;

// impl Eq for Square {}

// impl Display for Square {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match Self {
//             Piece => write!(f, "{}", Piece),
//             None => write!(f, "None")
//         }
//     }
// }
// Representation of a board is done via a vector of Pieces

pub struct Board {
    pub positions: [[Square;8];8],
    pub white_captured: Vec<Piece>, // Probably nice to know what has been captured in the future
    pub black_captured: Vec<Piece>,
    pub white_can_castle: bool,
    pub black_can_castle: bool 
}
pub const BASE_ROW : [Square;8] = [None, None, None, None, None, None, None, None];
impl Board {
    
    pub fn new() -> Board {

        Board {
            positions: [
                Board::gen_home_row(Colour::White).clone(),
                Board::gen_pawn_row(Colour::White).clone(),
                BASE_ROW.clone(),
                BASE_ROW.clone(),
                BASE_ROW.clone(),
                BASE_ROW.clone(),
                Board::gen_pawn_row(Colour::Black).clone(),
                Board::gen_home_row(Colour::Black).clone()
            ],
            white_captured: vec![],
            black_captured: vec![],
            white_can_castle: true,
            black_can_castle: true,
        }
    }

    pub fn gen_pawn_row(colour: Colour) -> [Square; 8]{
        let mut pawn_row = BASE_ROW.clone();
        for i in 0..8{
            pawn_row[i] = Some(create_pawn(colour.clone()))};
        pawn_row
    }

    pub fn gen_home_row(colour: Colour) -> [Square; 8]{
        [
            Some(create_rook(colour.clone())), 
            Some(create_knight(colour.clone())), 
            Some(create_bishop(colour.clone())), 
            Some(create_queen(colour.clone())), 
            Some(create_king(colour.clone())), 
            Some(create_bishop(colour.clone())), 
            Some(create_knight(colour.clone())), 
            Some(create_rook(colour.clone()))
        ]
    }

    pub fn display_string(&self) -> String {
        let mut display_string = String::from("");
        for row in &self.positions{
            let row_size = row.len();
            for (i,square) in row.iter().enumerate() {
                match square {
                    None => display_string.push_str(&String::from(format!("| _ "))),
                    Some(x) => display_string.push_str(&String::from(format!("| {} ", x)))
                }
                if i == row_size - 1 {
                    display_string.push('|')
                }
            }

            display_string.push_str("\n");
        }
        display_string
    }

    pub fn get_piece_at(&self, x: &u8, y: &u8) -> &Option<Piece>{
        return &self.positions[*y as usize][*x as usize];

    }

    pub fn move_piece_to(&mut self, old_x: &u8, old_y: &u8, new_x: &u8, new_y: &u8) {
        // Take a mutable reference copy of the piece to move
        // Once cloned, we no longer have the mutable reference
        
        let piece_to_move = &mut self.positions[*old_y as usize][*old_x as usize].clone();
        self.positions[*old_y as usize][*old_x as usize] = None;
        match piece_to_move {
            Some(piece) => {
                println!("{}", String::from(format!("new y: {}, new x: {}", new_y, new_x)));
                self.positions[*new_y as usize][*new_x as usize] = Some(piece.clone());
                drop(piece);
                drop(piece_to_move);
            }
            None => {
                drop(piece_to_move);
                return;
            }
        }

    }

    pub fn update_captured(&mut self, piece: Piece) {
        match (piece.colour) {
            Colour::White => {
                self.white_captured.push( piece.clone());
                return;
            }
            Colour::Black => {
                self.black_captured.push(piece.clone());
                return;
            }
        }

    }

    pub fn update_castlable(&mut self, moved_piece: &Piece) {
        if  moved_piece.name == "King" || moved_piece.name == "Rook" {
            match moved_piece.colour {
                Colour::White => {
                    self.white_can_castle = false
                }
                Colour::Black => {
                    self.black_can_castle = false
                }
            }
        }
    }

    pub fn can_castle(&self, colour: &Colour, rook_x: u8, king_x:u8) -> bool {
        let rank: i8;
        match colour {
            Colour::White => {
                if !self.white_can_castle{
                    return false;
                }
                rank = 0;
            }
            Colour::Black => {
                if !self.black_can_castle{
                    return false;
                }
                rank = 8;
            }
        }
        // Check if there is a clear line between rook and king, and neither of these pieces have moved yet
        let mut x_range: i8 = 0;
        x_range = (rook_x as i8 - king_x as i8).abs();

        for i in 1..x_range -1 {
            let square = &self.positions[i as usize][rank as usize];
            match square {
                Some(piece) => {
                    return false;
                }
                None => {}
            }
        }
        return true;
    }

    

}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.display_string())
    }
}

