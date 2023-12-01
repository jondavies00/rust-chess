use std::{fmt, error::Error};

#[derive(Debug, Clone)]
pub struct FormatMoveError;



impl Error for FormatMoveError {}
impl fmt::Display for FormatMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not format move, must be in format: (<file><rank> where file is a letter between a and h, and rank is an integer between 1 and 8, e.g. e4)")
    }

}

#[derive(Debug, Clone)]
pub struct InvalidMoveError;
impl Error for InvalidMoveError {}
impl fmt::Display for InvalidMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Move was invalid.")
    }

}