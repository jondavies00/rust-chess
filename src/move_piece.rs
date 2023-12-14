use crate::errors::FormatMoveError;

pub const FILES: [char;8] = ['a','b','c','d','e','f','g','h'];
#[derive(Clone)]
pub struct Move {
    pub x1: u8,
    pub y1: u8,
    pub x2: u8,
    pub y2: u8
}

pub fn format_coordinate(coord:String) -> Result<(u8, u8),FormatMoveError>  {
    let file = coord[0..1].parse::<char>();
    let rank = coord[1..2].parse::<u8>();


    match rank {
        Ok(rank) => {
            let rank = rank -1;

            match file {
                Ok(file) => {
                    let file_pos = FILES.iter().position(|r| *r == file);
                    match file_pos {
                        Some(file_pos) => {
                            return Ok((file_pos as u8, rank));
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

impl Move {
    pub fn from_string_coord(coord: String) -> Result<Move, FormatMoveError> {
        let coords = coord.split(" ").collect::<Vec<_>>();
        // for c in coords {
        //     c.clone().to_string();
        // }

        let from= format_coordinate(coords[0].clone().to_string());
        let to = format_coordinate(coords[1].clone().to_string());
        match from {
            Ok(from) => {
                match to {
                    Ok(to) => {
                        return Ok(Move {x1: from.0, y1 : from.1, x2: to.0, y2: to.1})
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }

            }
            Err(e) => {
                return Err(e);
            }
        }
        
        

    }
}

