use std::fmt;

#[derive(Debug)]
pub enum LineError {
    CompError(String),
    DestError(String),
    JumpError(String),
    InvalidSymbolError(String),
}

impl fmt::Display for LineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineError::CompError(ref sentence) => {
                write!(f, "Invalid Comp expression: {}", sentence)
            }
            LineError::DestError(ref sentence) => {
                write!(f, "Invalid Dest expression: {}", sentence)
            }
            LineError::JumpError(ref sentence) => {
                write!(f, "Invalid Jump expression: {}", sentence)
            }
            LineError::InvalidSymbolError(ref symbol) => {
                write!(f, "Invalid symbolname: {}", symbol)
            }
        }
    }
}

impl std::error::Error for LineError {}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot parse \n {}", self.message)
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    pub fn new() -> Self {
        Self {
            message: String::from(""),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.message.is_empty()
    }

    pub fn add(&mut self, errorkind: LineError, line_num: usize) {
        let msg = format!("{}: {}\n", line_num, errorkind);
        self.message = format!("{}{}", self.message, msg);
    }
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ParseError(ParseError),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::Io(error)
    }
}
