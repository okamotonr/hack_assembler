use std::fs::File;
use std::convert::TryFrom;
use std::io::{BufRead, BufReader};

use crate::error::{LineError};


#[derive(Debug, Clone, Copy)]
pub enum Comp {
    // Start at 0, b0xxxxxx
    Zero = 42,
    One = 63,
    MinusOne = 58,
    D = 12,
    A = 48,
    NotD = 13,
    NotA = 49,
    MinusD = 15,
    MinusA = 51,
    DpOne = 31,
    ApOne = 55,
    DmOne = 14,
    AmOne = 50,
    DpA = 2,
    DmA = 19,
    AmD = 7,
    DaA = 0,
    DoA = 21,
    // Start at 1, b1xxxxxx
    M = 112,
    NotM = 113,
    MinusM = 115,
    MpOne = 119,
    MmOne = 114,
    DpM = 66,
    DmM = 83,
    MmD = 71,
    DaM = 64,
    DoM = 85,
}

impl TryFrom<&str> for Comp {
    type Error = LineError;
    fn try_from(mnenonic: &str) -> Result<Self, LineError> {
        match mnenonic {
            "0" => Ok(Comp::Zero),
            "1" => Ok(Comp::One),
            "-1" => Ok(Comp::MinusOne),
            "D" => Ok(Comp::D),
            "A" => Ok(Comp::A),
            "!D" => Ok(Comp::NotD),
            "!A" => Ok(Comp::NotA),
            "-D" => Ok(Comp::MinusD),
            "-A" => Ok(Comp::MinusA),
            "D+1" => Ok(Comp::DpOne),
            "A+1" => Ok(Comp::ApOne),
            "D-1" => Ok(Comp::DmOne),
            "A-1" => Ok(Comp::AmOne),
            "D+A" => Ok(Comp::DpA),
            "D-A" => Ok(Comp::DmA),
            "A-D" => Ok(Comp::AmD),
            "D&A" => Ok(Comp::DaA),
            "D|A" => Ok(Comp::DoA),
            "M" => Ok(Comp::M),
            "!M" => Ok(Comp::NotM),
            "-M" => Ok(Comp::MinusM),
            "M+1" => Ok(Comp::MpOne),
            "M-1" => Ok(Comp::MmOne),
            "D+M" => Ok(Comp::DpM),
            "D-M" => Ok(Comp::DmM),
            "M-D" => Ok(Comp::MmD),
            "D&M" => Ok(Comp::DaM),
            "D|M" => Ok(Comp::DoM),
            _ => Err(LineError::CompError(mnenonic.to_string()))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Dest {
    // Start from 0
    Null,
    M,
    D,
    MD,
    A,
    AM,
    AD,
    AMD
}

impl TryFrom<&str> for Dest {
    type Error = LineError;
    fn try_from(mnenonic: &str) -> Result<Self, LineError> {
        match mnenonic {
                "M" => Ok(Dest::M),
                "D" => Ok(Dest::D),
                "MD" => Ok(Dest::MD),
                "A" => Ok(Dest::A),
                "AM" => Ok(Dest::AM),
                "AD" => Ok(Dest::AD),
                "AMD" => Ok(Dest::AMD),
                "" => Ok(Dest::Null),
                _ => Err(LineError::CompError(mnenonic.to_string()))
            }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Jump {
    // Start from 0
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP
}

impl TryFrom<&str> for Jump {
    type Error = LineError;
    fn try_from(mnenonic: &str) -> Result<Self, LineError> {
        match mnenonic {
            "JGT" => Ok(Jump::JGT),
            "JEQ" => Ok(Jump::JEQ),
            "JGE" => Ok(Jump::JGE),
            "JLT" => Ok(Jump::JLT),
            "JNE" => Ok(Jump::JNE),
            "JLE" => Ok(Jump::JLE),
            "JMP" => Ok(Jump::JMP),
            "" => Ok(Jump::Null),
            _ => Err(LineError::JumpError(mnenonic.to_string()))
        }
    }
}



#[derive(Debug)]
pub enum AsmLine {
    ACommand(String),
    CCommand(Comp, Dest, Jump),
    LCommand(String),
}

impl AsmLine {
    pub fn symbol(&self) -> Option<String> {
        match self {
            AsmLine::ACommand(ref symbol) | AsmLine::LCommand(ref symbol) => Some(symbol.clone()),
            _ => None,
        }
    }
    pub fn comp(&self) -> Option<Comp> {
        match self {
            AsmLine::CCommand(comp, _, _) => Some(*comp),
            _ => None
        }
    }

    pub fn dest(&self) -> Option<Dest> {
        match self {
            AsmLine::CCommand(_, dest, _) => Some(*dest),
            _ => None
        }
    }

    pub fn jump(&self) -> Option<Jump> {
        match self {
            AsmLine::CCommand(_, _, jump) => Some(*jump),
            _ => None
        }
    }
}

pub struct Parser {
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, path: &str) -> Result<Vec<AsmLine>, std::io::Error> {
        let mut shoud_panic = false;
        let mut lines: Vec<AsmLine> = Vec::new();
        let file = File::open(path)?;
        for (index, result) in BufReader::new(file).lines().enumerate() {
            // parse white space
            let raw_line = result?.replace(" ", "");
            // parse comment
            let v: Vec<&str> = raw_line.split("//").collect();
            let line = v[0];
            if line.is_empty() {
                continue
            }
            let mnenonic = self.parse_line(line);
            if let Err(e) = mnenonic {
                eprintln!("{}: {}", index, e);
                shoud_panic = true;
                continue
            }
            lines.push(mnenonic.unwrap())
        }
        if shoud_panic {
            panic!("Syntax Error...")
        }

        Ok(lines)

    }
    fn parse_line(&self, line: &str) -> Result<AsmLine, LineError> {
        if line.starts_with("@") {
            self.parse_a_command(line)
        } else if line.starts_with("(") {
            self.parse_l_command(line)
        } else {
            self.parse_c_command(line)
        }
    } 

    fn parse_a_command(&self, line: &str) -> Result<AsmLine, LineError> {
        let allowed_len = line.chars().count() - 1;
        let symbol = line.replace("@", "");
        if allowed_len != symbol.chars().count() {
            return Err(LineError::InvalidSymbolError(symbol))
        }

        if self.validate_syboml(&symbol) {
            return Ok(AsmLine::ACommand(symbol))
        } else {
            return Err(LineError::InvalidSymbolError(symbol))
        }
    }

    fn parse_l_command(&self, line: &str) -> Result<AsmLine, LineError> {
        let allowed_len = line.chars().count() - 2;
        let symbol = line.replace("(", "").replace(")", "");
        if allowed_len != symbol.chars().count() {
            return Err(LineError::InvalidSymbolError(symbol))
        }

        if self.validate_syboml(&symbol) {
            return Ok(AsmLine::LCommand(symbol))
        } else {
            return Err(LineError::InvalidSymbolError(symbol))
        }

    }

    fn parse_c_command(&self, line: &str) -> Result<AsmLine, LineError> {
        let comp = self.get_comp(line)?;
        let dest = self.get_dest(line)?;
        let jump = self.get_jump(line)?;

        Ok(AsmLine::CCommand(comp, dest, jump))
    }

    fn validate_syboml(&self, symbol: &str) -> bool {
        let ch_1 = symbol.chars().nth(0).unwrap();
        let allowed_sign = ['_', '.', '$', ':'];

        if ch_1.is_numeric(){
            if symbol.chars().all(char::is_numeric) {
                return true
            } else {
                return false
            }

        } else {
            for ch in symbol.chars() {
                if let false = ch.is_ascii_alphanumeric() {
                    if let false = allowed_sign.contains(&ch) {
                        return false
                    }
                }
            }
            return true
        }
    }

    fn get_comp(&self, line: &str) -> Result<Comp, LineError> {
        let v: Vec<&str> = line.split("=").collect();
        let comp = if v.len() == 2 { v[1] } else { v[0] };
        let v: Vec<&str> = comp.split(";").collect();
        let comp = v[0];
        Comp::try_from(comp)
    }


    fn get_dest(&self, line: &str) -> Result<Dest, LineError> {
        let v: Vec<&str> = line.split("=").collect();
        let len = v.len();

        match len {
            1 => Dest::try_from(""),
            2 => Dest::try_from(v[0]),
            _ => Err(LineError::DestError(line.to_string())),
        }
    }

    fn get_jump(&self, line: &str) -> Result<Jump, LineError> {
        let v: Vec<&str> = line.split(";").collect();
        let len = v.len();

        match len {
            1 => Jump::try_from(""),
            2 => Jump::try_from(v[1]),
            _ => Err(LineError::JumpError(line.to_string())),
        }
    }
}


