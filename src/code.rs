use std::collections::HashMap;

use crate::parser::AsmLine;

#[derive(Debug)]
struct SymbolTable {
    table: HashMap<String, u32>,
    rom_address: u32,
    index: u32,
    limit: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        let mut index = 0;
        let defalut_symbol = vec!["SP", "LCL", "ARG", "THIS", "THAT"];

        for (value, name) in (0_u32..).zip(defalut_symbol.iter()) {
            table.insert(name.to_string(), value);
        }

        let defalut_symbol = [
            "R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12", "R13",
            "R14", "R15",
        ];

        for (value, name) in (0_u32..).zip(defalut_symbol.iter()) {
            table.insert(name.to_string(), value);
            index = value;
        }

        index += 1;
        let limit = 16384;
        table.insert("SCREEN".to_string(), limit);
        table.insert("KBD".to_string(), 24576);
        let rom_address = 0;
        SymbolTable {
            table,
            rom_address,
            index,
            limit,
        }
    }

    pub fn insert(&mut self, name: String) {
        if self.index == self.limit {
            panic!("You cannot insert symbol anymore");
        }
        if name.chars().all(char::is_numeric) {
            return;
        }
        if self.table.get(&name).is_none() {
            self.table.insert(name, self.index);
            self.index += 1;
        }
    }

    pub fn get(&self, name: String) -> Option<u32> {
        if name.chars().all(char::is_numeric) {
            let ret: u32 = name.parse().unwrap();
            return Some(ret);
        }
        let ret = self.table.get(&name);
        match ret {
            Some(_) => Some(*ret.unwrap()),
            None => None,
        }
    }

    pub fn set_rom(&mut self, line: &AsmLine) {
        match line {
            AsmLine::LCommand(_) => self.insert_rom(line.symbol().unwrap()),
            _ => self.rom_address += 1,
        }
    }

    fn insert_rom(&mut self, name: String) {
        self.table.insert(name, self.rom_address);
    }
}

#[derive(Debug)]
pub struct CodeGenerator {
    table: SymbolTable,
    lines: Vec<AsmLine>,
}

impl CodeGenerator {
    pub fn new(lines: Vec<AsmLine>) -> Self {
        let mut table = SymbolTable::new();

        for line in lines.iter() {
            table.set_rom(line);
        }
        for line in lines.iter() {
            if line.symbol().is_some() {
                table.insert(line.symbol().unwrap());
            }
        }
        CodeGenerator { table, lines }
    }

    pub fn gen(&self) {
        let mut ret: Vec<String> = Vec::new();
        for line in self.lines.iter() {
            let code = self.translate(line);
            ret.push(code);
        }
        for line in ret {
            print!("{}", line)
        }
    }
    fn translate(&self, line: &AsmLine) -> String {
        match line {
            AsmLine::CCommand(_, _, _) => self.translate_c_command(line),
            AsmLine::ACommand(_) => self.translate_a_command(line),
            AsmLine::LCommand(_) => self.translate_l_command(),
        }
    }
    fn translate_c_command(&self, line: &AsmLine) -> String {
        let mut base_str = "111".to_string();
        let comp = &format!("{:07b}", line.comp().unwrap() as u32);
        let dest = &format!("{:03b}", line.dest().unwrap() as u32);
        let jump = &format!("{:03b}\n", line.jump().unwrap() as u32);
        base_str = base_str + comp + dest + jump;
        base_str
    }
    fn translate_a_command(&self, line: &AsmLine) -> String {
        let address = self.table.get(line.symbol().unwrap());
        let b_str: String = format!("{:016b}\n", address.unwrap());
        b_str
    }
    fn translate_l_command(&self) -> String {
        "".to_string()
    }
}
