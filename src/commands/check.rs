use crate::{commands::utils::read_file, parser::parse, tokenizer::tokenize};
use log::debug;

pub fn check(file: &str) {
    let source = read_file(file);

    println!("Checking the file {file}");

    let tokens = tokenize(source);

    debug!("Parsing the tokens");
    parse(tokens);
    debug!("Done parsing");

    println!("No issues found with the file {file}");
}
