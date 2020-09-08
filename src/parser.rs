use std::fs::File;
use std::convert::From;
use std::io::{BufRead, BufReader};


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

impl From<&str> for Comp {
    fn from(mnenonic: &str) -> Self {
        match mnenonic {
            "0" => Comp::Zero,
            "1" => Comp::One,
            "-1" => Comp::MinusOne,
            "D" => Comp::D,
            "A" => Comp::A,
            "!D" => Comp::NotD,
            "!A" => Comp::NotA,
            "-D" => Comp::MinusD,
            "-A" => Comp::MinusA,
            "D+1" => Comp::DpOne,
            "A+1" => Comp::ApOne,
            "D-1" => Comp::DmOne,
            "A-1" => Comp::AmOne,
            "D+A" => Comp::DpA,
            "D-A" => Comp::DmA,
            "A-D" => Comp::AmD,
            "D&A" => Comp::DaA,
            "D|A" => Comp::DoA,
            "M" => Comp::M,
            "!M" => Comp::NotM,
            "-M" => Comp::MinusM,
            "M+1" => Comp::MpOne,
            "M-1" => Comp::MmOne,
            "D+M" => Comp::DpM,
            "D-M" => Comp::DmM,
            "M-D" => Comp::MmD,
            "D&M" => Comp::DaM,
            "D|M" => Comp::DoM,
            _ => panic!("Not match")
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

impl From<&str> for Dest {
    fn from(mnenonic: &str) -> Self {
        match mnenonic {
                "M" => Dest::M,
                "D" => Dest::D,
                "MD" => Dest::MD,
                "A" => Dest::A,
                "AM" => Dest::AM,
                "AD" => Dest::AD,
                "AMD" => Dest::AMD,
                "" => Dest::Null,
                _ => panic!("OOPS"),
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

impl From<&str> for Jump {
    fn from(mnenonic: &str) -> Self {
        match mnenonic {
            "JGT" => Jump::JGT,
            "JEQ" => Jump::JEQ,
            "JGE" => Jump::JGE,
            "JLT" => Jump::JLT,
            "JNE" => Jump::JNE,
            "JLE" => Jump::JLE,
            "JMP" => Jump::JMP,
            "" => Jump::Null,
            _ => panic!("Ooops")
        }
    }
}

#[derive(Debug)]
pub enum AsmLine {
    ACommand(String),
    CCommand(Comp, Dest, Jump),
    LCommand(String),
}

impl From<&str> for AsmLine{
    fn from(line: &str) -> Self {
        if line.starts_with("(") {
            let symbol = line.replace("(", "").replace(")", "");
            return AsmLine::LCommand(symbol)
        }
        if line.starts_with("@") {
            let symbol = line.replace("@", "");
            return AsmLine::ACommand(symbol)
        }
        let comp = AsmLine::get_comp(line);
        let dest = AsmLine::get_dest(line);
        let jump = AsmLine::get_jump(line);

        AsmLine::CCommand(comp, dest, jump)
    }
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


    fn get_comp(line: &str) -> Comp {
        let v: Vec<&str> = line.split("=").collect();
        let comp = if v.len() == 2 { v[1] } else { v[0] };
        let v: Vec<&str> = comp.split(";").collect();
        let comp = v[0];
        Comp::from(comp)
        }

    fn get_dest(line: &str) -> Dest {
        let v: Vec<&str> = line.split("=").collect();
        let len = v.len();

        match len {
            1 => Dest::from(""),
            2 => Dest::from(v[0]),
            _ => panic!("oooops"),
        }
    }

    fn get_jump(line: &str) -> Jump {
        let v: Vec<&str> = line.split(";").collect();
        let len = v.len();

        match len {
            1 => Jump::from(""),
            2 => Jump::from(v[1]),
            _ => panic!("oooops"),
        }
    }

}

pub struct Parser {
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
    pub fn parse(path: &str) -> Result<Vec<AsmLine>, Box<std::error::Error>> {
        let mut lines: Vec<AsmLine> = Vec::new();
        let file = File::open(path)?;
        for result in BufReader::new(file).lines() {
            // parse white space
            let raw_line = result?.replace(" ", "");
            // parse comment
            let v: Vec<&str> = raw_line.split("//").collect();
            let line = v[0];
            if line.is_empty() {
                continue
            }
            let mnenonic = AsmLine::from(line);
            lines.push(mnenonic)
        }

        Ok(lines)

    }
}

