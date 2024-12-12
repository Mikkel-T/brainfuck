use crate::{commands::utils::read_file, interpreter, parser::parse, tokenizer::tokenize};
use log::debug;

pub fn run(file: &str) {
    let source = read_file(file);

    let tokens = tokenize(source);
    debug!("Parsing the tokens");
    let instructions = parse(tokens);
    debug!("Done parsing");

    let mut tape: [u8; 30000] = [0; 30000];
    let mut ptr = 0;
    debug!("Running the program");

    interpreter::run(&instructions, &mut tape, &mut ptr);
}
