use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct ParseError{
    pub kind: ParseErrorKind
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl ParseError {
    pub fn new(kind: ParseErrorKind) -> Self {
        Self{ kind }
    }
}


#[derive(Debug)]
pub enum ParseErrorKind {
    CompError(String),
    DestError(String),
    JumpError(String),
    InvalidSymbolError(String)
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErrorKind::CompError(ref sentence) => {
                write!(f, "Invalid Comp expression: {}", sentence)
            },
            ParseErrorKind::DestError(ref sentence) => {
                write!(f, "Invalid Dest expression: {}", sentence)
            },
            ParseErrorKind::JumpError(ref sentence) => {
                write!(f, "Invalid Jump expression: {}", sentence)
            },
            ParseErrorKind::InvalidSymbolError(ref symbol) => {
                write!(f, "Invalid symbolname: {}", symbol)
            }
        }
    }
}


