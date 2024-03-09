use std::{fmt::{Display, Formatter, Result}, ops::Range};


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
            }
            None => {

            }
        }

    }

    pub fn update_captured(&mut self, move_x: &u8, move_y: &u8) {
        let piece = &self.positions[*move_y as usize][*move_x as usize];
        match (piece) {
            Some(piece) => {
                 match (piece.colour) {
                    Colour::White => {
                        self.white_captured.push( piece.clone());
                    }
                    Colour::Black => {
                        self.black_captured.push(piece.clone());
                    }
                }
            }
            None => {}
        }

    }

    pub fn update_castleable(&mut self, move_x: &u8, move_y: &u8) {
        
        let piece = &self.positions[*move_y as usize][*move_x as usize];
        match (piece){
            Some(piece) => {
            if  piece.name == "King" || piece.name == "Rook" {
                match piece.colour {
                    Colour::White => {
                        self.white_can_castle = false;
                        return;
                    }
                    Colour::Black => {
                        self.black_can_castle = false;
                        return;
                    }
                }
            }}
            None => {}
        }
    }

    // pub fn is_valid_castle(&self, move_x1: &u8, move_x2: &u8, colour: Colour) {
    //     // We know king + rook of same colour are selected
    //     // Calculate if its king/queen side and return true if they are valid
    //     if move_x == 0

    // }

    pub fn can_castle(&self, colour: &Colour, rook_x: &u8, king_x:&u8) -> bool {
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
        println!("{}", String::from("Checking in between"));
        // Check if there is a clear line between rook and king, and neither of these pieces have moved yet
        let range: Range<u8>;

        if rook_x > king_x {
            range = king_x + 1..rook_x - 1;
        }
        else {
            range = rook_x + 1..king_x - 1;
        }
        for i in range {
            let square: &Option<Piece> = &self.positions[rank as usize][i as usize];
            println!("{}", String::from(format!("Checking square coords at {}, {}", i, rank)));
            match square {
                Some(_piece) => {
                    return false;
                }
                None => {}
            }
        }
        return true;
    }

    // pub fn update_can_castle(&mut self) {
    //     let check_tuple = vec![((0, 4), (0, 7), Colour::White), ((0, 4), (0, 0), Colour::White), ((7, 4), (7, 7), Colour::Black), ((0, 4), (7, 0), Colour::Black)];

    //     // Match any piece is None
    //     for c in check_tuple {
    //         let y = c.0.0 as usize;
    //         let x = c.0.1 as usize;
    //         let piece = &self.positions[y][x];
    //         match piece {
    //             Some(_piece) => {}
    //             None => {continue;}
    //         }
    //         match c.2 {
    //             Colour::White => {
    //                 self.white_can_castle = self.can_castle(&c.2, &c.1.1, &c.0.1);
    //             }
    //             Colour::Black => {self.black_can_castle = self.can_castle(&c.2, &c.1.1, &c.0.1);}
    //         }
            
    //     }
    // }

    pub fn castle(&mut self, king_x: &u8, king_y: &u8, rook_x: &u8, rook_y: &u8, colour: &Colour){
        // Check if its a kingside or queenside castle
        let king = self.positions[*king_y as usize][*king_x as usize].clone();
        let rook = self.positions[*rook_y as usize][*rook_x as usize].clone();
        // let king_y = *king_y as usize;
        // let king_x = *king_x as usize;
        // let rook_y = *rook_y as usize;
        // let rook_x = *rook_x as usize;
        match king {
            Some(king) => {
                match rook {
                    Some(rook) => {
                        match rook_x {
                            7 => {
                                // Kingside: king to 6, rook to 5
                                self.positions[*king_y as usize][*king_x as usize] = None;
                                self.positions[*king_y as usize][6] = Some(king);
                                
                                self.positions[*rook_y as usize][*rook_x as usize] = None;
                                self.positions[*rook_y as usize][5] = Some(rook);
                            }
                            4 => {
                                //Queenside: king to 3, rook to 4
                                self.positions[*king_y as usize][*king_x as usize] = None;
                                self.positions[*king_y as usize][3] = Some(king);
                
                                self.positions[*rook_y as usize][*rook_x as usize] = None;
                                self.positions[*rook_y as usize][4] = Some(rook);
                            }
                            _ => {}
                        }

                    }
                    None => {}
                }
            }
            None => {}
        }

        match colour {
            Colour::White => {
                self.white_can_castle = false;
            }
            Colour::Black => {
                self.black_can_castle = false;
            }
        }
        
    }   

    

}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.display_string())
    }
}

