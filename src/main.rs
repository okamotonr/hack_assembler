extern crate hack_assembler;

use std::env;
use std::fs::File;
use std::io::{BufReader, Error};

use hack_assembler::code;
use hack_assembler::parser;

fn argparse(args: &[String]) -> Result<&str, &'static str> {
    let length = args.len();
    match length {
        2 => Ok(&args[1]),
        _ => Err("usage $file_path"),
    }
}

fn file_parser(file_path: &str) -> Result<BufReader<File>, Error> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = file_parser(argparse(&args).unwrap()).unwrap();
    let hack_parser = parser::Parser::new();
    let lines = hack_parser.parse(reader).unwrap();
    let code_generator = code::CodeGenerator::new(lines);
    code_generator.gen();
}
