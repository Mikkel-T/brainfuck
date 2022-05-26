use crate::tokenizer::Token;

pub enum Instruction {
    Right,
    Left,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

pub fn parse(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, token) in tokens.iter().enumerate() {
        if loop_stack == 0 {
            match token {
                Token::Right => instructions.push(Instruction::Right),
                Token::Left => instructions.push(Instruction::Left),
                Token::Increment => instructions.push(Instruction::Increment),
                Token::Decrement => instructions.push(Instruction::Decrement),
                Token::Write => instructions.push(Instruction::Write),
                Token::Read => instructions.push(Instruction::Read),
                Token::LoopStart => {
                    loop_stack += 1;
                    loop_start = i;
                }
                Token::LoopEnd => panic!("The loop ending at {} has no starting point", i),
            }
        } else {
            match token {
                Token::LoopStart => loop_stack += 1,
                Token::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        instructions
                            .push(Instruction::Loop(parse(tokens[loop_start + 1..i].to_vec())));
                    }
                }
                _ => {}
            }
        }
    }

    if loop_stack != 0 {
        panic!("The loop starting at {} has no ending point", loop_start);
    }

    return instructions;
}
