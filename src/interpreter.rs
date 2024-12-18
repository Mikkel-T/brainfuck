use crate::parser::Instruction;
use log::error;
use std::{
    io::{stdout, Read, Write},
    process,
};

/// Run a vec of instructions generated by the parser
pub fn run(instructions: &Vec<Instruction>, tape: &mut [u8; 30000], ptr: &mut usize) {
    for instruction in instructions {
        match instruction {
            Instruction::Right => {
                if *ptr == 29999 {
                    error!("Pointer out of bounds: Length of tape is 30000");
                    process::exit(1);
                }

                *ptr += 1;
            }

            Instruction::Left => {
                if *ptr == 0 {
                    error!("Pointer out of bounds: Pointer can not be less than 0");
                    process::exit(1);
                }

                *ptr -= 1;
            }

            Instruction::Increment => {
                tape[*ptr] = tape[*ptr].wrapping_add(1);
            }

            Instruction::Decrement => {
                tape[*ptr] = tape[*ptr].wrapping_sub(1);
            }

            Instruction::Write => {
                print!("{}", tape[*ptr] as char);
                stdout().flush().expect("Could not flush stdout");
            }

            Instruction::Read => {
                let input: Option<u8> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok());
                if let Some(new) = input {
                    tape[*ptr] = new
                };
            }

            Instruction::Loop(program) => {
                while tape[*ptr] != 0 {
                    run(program, tape, ptr);
                }
            }
        }
    }
}
