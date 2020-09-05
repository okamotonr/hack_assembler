use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug)]
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

impl Comp {
    pub fn from_string(nimonick: &str) -> Self {
        match nimonick {
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

#[derive(Debug)]
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

impl Dest {
    pub fn from_string(nimonick: Option<&str>) -> Self {
        if nimonick.is_some() {
            let nimonick: &str = nimonick.unwrap();
            match nimonick {
                "M" => Dest::M,
                "D" => Dest::D,
                "MD" => Dest::MD,
                "A" => Dest::A,
                "AM" => Dest::AM,
                "AD" => Dest::AD,
                "AMD" => Dest::AMD,
                _ => panic!("OOPS"),
            }
        } else {
            Dest::Null
        }
    }
}

#[derive(Debug)]
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
impl Jump {
    pub fn from_string(nimonick: Option<&str>) -> Self {
        if nimonick.is_some() {
            let nimonick: &str = nimonick.unwrap();
            match nimonick {
                "JGT" => Jump::JGT,
                "JEQ" => Jump::JEQ,
                "JGE" => Jump::JGE,
                "JLT" => Jump::JLT,
                "JNE" => Jump::JNE,
                "JLE" => Jump::JLE,
                "JMP" => Jump::JMP,
                _ => panic!("Ooops")
            }
        } else {
            Jump::Null
        }
    }
}

#[derive(Debug)]
pub enum CommandType {
    ACommand,
    CCommand,
    LCommand,
}

impl CommandType{
    pub fn from_line(line: &str) -> Self {
        if line.starts_with("(") {
            return Self::LCommand
        }
        if line.starts_with("@") {
            return Self::ACommand
        }
        Self::CCommand
    }
}

#[derive(Debug)]
pub struct AsmLine {
    pub line: String,
    pub command_t: CommandType,
}

impl AsmLine {
    pub fn new(line: &str) -> Self {
        let command_t = CommandType::from_line(line);
        let line = line.to_string();

        Self { line, command_t }
    }

    pub fn symbol(&self) -> Option<String> {
        match self.command_t {
            CommandType::CCommand => None,
            CommandType::ACommand => Some(self.line.replace("@", "")),
            CommandType::LCommand => Some(self.line.replace("(", "").replace(")", ""))
        }
    }
    pub fn comp(&self) -> Option<Comp> {
        match self.command_t {
            CommandType::CCommand => Some(self.get_comp()),
            _ => None
        }
    }

    pub fn dest(&self) -> Option<Dest> {
        match self.command_t {
            CommandType::CCommand => Some(self.get_dest()),
            _ => None
        }
    }

    pub fn jump(&self) -> Option<Jump> {
        match self.command_t {
            CommandType::CCommand => Some(self.get_jump()),
            _ => None
        }
    }


    fn get_comp(&self) -> Comp {
        let v: Vec<&str> = self.line.split("=").collect();
        let comp = if v.len() == 2 { v[1] } else { v[0] };
        let v: Vec<&str> = comp.split(";").collect();
        let comp = v[0];
        Comp::from_string(comp)
        }

    fn get_dest(&self) -> Dest {
        let v: Vec<&str> = self.line.split("=").collect();
        let len = v.len();

        match len {
            1 => Dest::from_string(None),
            _ => Dest::from_string(Some(v[0]))
        }
    }

    fn get_jump(&self) -> Jump {
        let v: Vec<&str> = self.line.split(";").collect();
        let len = v.len();

        match len {
            1 => Jump::from_string(None),
            _ => Jump::from_string(Some(v[1]))
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
            let nimonick = AsmLine::new(line);
            lines.push(nimonick)
        }

        Ok(lines)

    }
}

