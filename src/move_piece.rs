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
            if rank == 0 || rank >= 9 {
                println!("{}", String::from("Invalid Rank!"));
                return Err(FormatMoveError)
            }
            let rank = rank -1;

            match file {
                Ok(file) => {
                    let file_pos = FILES.iter().position(|r| *r == file);
                    match file_pos {
                        Some(file_pos) => {

                            return Ok((file_pos as u8, rank));
                        },
                        None => {
                            println!("{}", String::from("Invalid File!"));
                            return Err(FormatMoveError);
                        }
                    }},
                Err(file) => {println!("{}", String::from("Invalid File!"));return Err(FormatMoveError)}
            }
        
        },
        Err(rank) => {println!("{}", String::from("Invalid Rank!")); return Err(FormatMoveError)}
        
    }

}

impl Move {
    pub fn from_string_coord(coord: String) -> Result<Move, FormatMoveError> {
        
        let coords = coord.split(" ").collect::<Vec<_>>();
        if coords.len() != 2 {
            println!("{}", String::from("Invalid number of coordinates!"));
            return Err(FormatMoveError);
        }
        // println!("{}", String::from(format!("coords {}", coords[1])));
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

    pub fn from_int_coord(x1: u8, x2: u8, y1:u8, y2: u8) -> Move{
        return Move{x1: x1, y1: y1, x2: x2, y2: y2};
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_invalid_number_of_coords() {
        // let expected = FormatMoveError;
        let formatted = Move::from_string_coord(String::from("x0"));

        let expected = FormatMoveError;
        assert!(matches!(formatted, Err(expected)));

    }
    #[test]
    fn test_format_invalid_file() {
        // let expected = FormatMoveError;
        let formatted = Move::from_string_coord(String::from("a2 x1"));

        let expected = FormatMoveError;
        assert!(matches!(formatted, Err(expected)));

    }
    #[test]
    fn test_format_invalid_rank() {
        // let expected = FormatMoveError;
        let formatted = Move::from_string_coord(String::from("a2 a9"));

        let expected = FormatMoveError;
        assert!(matches!(formatted, Err(expected)));

    }
    #[test]
    fn test_format_invalid_rank_negative() {
        // let expected = FormatMoveError;
        let formatted = Move::from_string_coord(String::from("a-2 a2"));

        let expected = FormatMoveError;
        assert!(matches!(formatted, Err(expected)));

    }
    #[test]
    fn test_format_coord() {
        let expected: Move = Move{x1:4, x2: 4, y1: 1, y2: 3};
        let formatted = Move::from_string_coord(String::from("e2 e4")).unwrap();
        assert_eq!(expected.x1, formatted.x1);
        assert_eq!(expected.x2, formatted.x2);
        assert_eq!(expected.y1, formatted.y1);
        assert_eq!(expected.y2, formatted.y2);
    }
}