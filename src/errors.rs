use std::fmt;

#[derive(Debug, Clone)]
pub struct FormatMoveError;

impl fmt::Display for FormatMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid move, must be in format: (<file><rank> where file is a letter between a and h, and rank is an integer between 1 and 8, e.g. e4)")
    }

}