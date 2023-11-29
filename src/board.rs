use std::fmt::{Display, Formatter, Result};
#[path = "pieces.rs"] mod pieces;

use pieces::{Piece, Colour, create_pawn, create_bishop, create_king, create_knight, create_rook, create_queen};

// pub enum Position {
//     Piece,
//     None
// }

type Square = Option<pieces::Piece>;

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
    positions: [[Square;8];8],
}
const BASE_ROW : [Square;8] = [None, None, None, None, None, None, None, None];
impl Board {
    
    pub fn new() -> Board {

        Board {
            positions: [
                Board::gen_home_row(Colour::Black).clone(),
                Board::gen_pawn_row(Colour::Black).clone(),
                BASE_ROW.clone(),
                BASE_ROW.clone(),
                BASE_ROW.clone(),
                BASE_ROW.clone(),
                Board::gen_pawn_row(Colour::White).clone(),
                Board::gen_home_row(Colour::White).clone()
            ]
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
        return &self.positions[*x as usize][*y as usize];

    }

    pub fn move_piece_to(&mut self, old_x: &u8, old_y: &u8, new_x: &u8, new_y: &u8) {
        // Take a mutable reference copy of the piece to move
        // Once cloned, we no longer have the mutable reference
        let piece_to_move = &mut self.positions[*old_x as usize][*old_y as usize].clone();
        self.positions[*old_x as usize][*old_y as usize] = None;
        match piece_to_move {
            Some(piece) => {
                self.positions[*new_x as usize][*new_y as usize] = Some(piece.clone());
            }
            None => {
                return;
            }
        }

    }


    

}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.display_string())
    }
}

