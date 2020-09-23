extern crate hack_assembler;

use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader, Error};
use std::path::{Path, PathBuf};

use hack_assembler::code;
use hack_assembler::parser;

fn get_answer<T: AsRef<Path>>(q_path: T) -> PathBuf {
    q_path.as_ref().with_extension("hack")
    // let parent = dir_path.parent().unwrap();
    // let stem = dir_path.file_stem().unwrap();
    // let extension = ".hack";
    // let file_name = format!("{}{}", stem.to_str().unwrap(), extension);
    // parent.join(file_name)
}

fn get_reader<T: AsRef<Path>>(path: T) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn get_questions(root_dir: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for path in read_dir(root_dir).unwrap() {
        let f_o_d = path.unwrap().path();
        if f_o_d.is_dir() {
            files.append(&mut get_questions(f_o_d.to_str().unwrap()));
        } else {
            match f_o_d.extension() {
                Some(os_ext) => match os_ext.to_str() {
                    Some("asm") => files.push(f_o_d.display().to_string()),
                    _ => continue,
                },
                None => continue,
            }
        }
    }
    files
}

fn get_expected<T: BufRead>(reader: T) -> Vec<String> {
    let mut ret = Vec::new();
    for result in reader.lines() {
        let raw_line = result.unwrap();
        ret.push(raw_line);
    }

    ret
}

fn test_impl(q_path: &str) {
    let a_path = get_answer(q_path);
    if !a_path.exists() {
        panic!("{:?} is not exists", a_path)
    };
    let expected = get_expected(get_reader(a_path).unwrap());
    let hack_parser = parser::Parser::new();
    let parsed = hack_parser.parse(get_reader(q_path).unwrap()).unwrap();
    let generator = code::CodeGenerator::new(parsed);
    let result = generator.gen();
    assert_eq!(result, expected)
}

#[test]
fn test_of_integration() {
    let root_dir = "tests/resource";
    let questions = &get_questions(root_dir);
    for q_path in questions {
        test_impl(q_path)
    }
}
