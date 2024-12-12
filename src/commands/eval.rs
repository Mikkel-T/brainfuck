use crate::{interpreter, parser::parse, tokenizer::tokenize};

pub fn eval(code: &str) {
    let tokens = tokenize(code.to_string());
    let instructions = parse(tokens);

    let mut tape: [u8; 30000] = [0; 30000];
    let mut ptr = 0;

    interpreter::run(&instructions, &mut tape, &mut ptr);
}
