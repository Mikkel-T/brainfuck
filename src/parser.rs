use crate::tokenizer::{Position, Token};
use log::error;
use std::process;

/// Brainfuck instructions that can be run by the interpreter
pub enum Instruction {
    /// Move pointer right
    Right,
    /// Move pointer left
    Left,
    /// Increment current cell
    Increment,
    /// Decrement current cell
    Decrement,
    /// Output value of current cell
    Write,
    /// Replace value of current cell with input
    Read,
    /// Loop over vec of instructions while current cell is not 0
    Loop(Vec<Instruction>),
}

/// Parses a vec of tokens to a vec of instructions
pub fn parse(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;
    let mut loop_start_pos = Position { line: 0, col: 0 };

    for (i, token) in tokens.iter().enumerate() {
        if loop_stack == 0 {
            match token {
                Token::Right => instructions.push(Instruction::Right),
                Token::Left => instructions.push(Instruction::Left),
                Token::Increment => instructions.push(Instruction::Increment),
                Token::Decrement => instructions.push(Instruction::Decrement),
                Token::Write => instructions.push(Instruction::Write),
                Token::Read => instructions.push(Instruction::Read),
                Token::LoopStart(pos) => {
                    loop_stack += 1;
                    loop_start = i;
                    loop_start_pos = pos.clone();
                }
                Token::LoopEnd(pos) => {
                    error!("SyntaxError: The loop ending at {pos} has no starting point");
                    process::exit(1);
                }
            }
        } else {
            match token {
                Token::LoopStart(_) => loop_stack += 1,
                Token::LoopEnd(_) => {
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
        error!("SyntaxError: The loop starting at {loop_start_pos} has no ending point");
        process::exit(1);
    }

    return instructions;
}
