use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub enum LineError {
    CompError(String),
    DestError(String),
    JumpError(String),
    InvalidSymbolError(String)
}

impl fmt::Display for LineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineError::CompError(ref sentence) => {
                write!(f, "Invalid Comp expression: {}", sentence)
            },
            LineError::DestError(ref sentence) => {
                write!(f, "Invalid Dest expression: {}", sentence)
            },
            LineError::JumpError(ref sentence) => {
                write!(f, "Invalid Jump expression: {}", sentence)
            },
            LineError::InvalidSymbolError(ref symbol) => {
                write!(f, "Invalid symbolname: {}", symbol)
            }
        }
    }
}

impl Error for LineError {}
