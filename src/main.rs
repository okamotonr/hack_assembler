extern crate hack_assembler;

use std::env;

use hack_assembler::parser;
use hack_assembler::code;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let hack_parser = parser::Parser::new();
    
    let lines = hack_parser.parse(file_path).unwrap();
    let code_generator = code::CodeGenerator::new(lines);
    code_generator.gen();
}
