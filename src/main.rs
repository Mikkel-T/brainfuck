mod parser;
pub mod tokenizer;

use clap::{Parser, Subcommand};
use parser::{parse, Instruction};
use std::fs;
use std::io::{stdout, Read, Write};
use std::process;
use tokenizer::tokenize;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Runs a brainfuck program
    Run {
        /// The brainfuck program to run
        file: String,
    },
    /// Checks the syntax of a brainfuck file without running it
    Check {
        /// The brainfuck program to check
        file: String,
    },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Run { file } => {
            let source = fs::read_to_string(&file).unwrap_or_else(|err| {
                println!("couldn't read {}: {}", &file, err);
                process::exit(1);
            });

            let tokens = tokenize(source);
            let instructions = parse(tokens);

            let mut tape: [u8; 30000] = [0; 30000];
            let mut ptr = 0;
            run(&instructions, &mut tape, &mut ptr)
        }
        Commands::Check { file } => {
            let source = fs::read_to_string(&file).unwrap_or_else(|err| {
                println!("couldn't read {}: {}", &file, err);
                process::exit(1);
            });

            println!("Checking the file {}", &file);

            parse(tokenize(source));
            println!("No issues found with the file {}", &file);
        }
    }
}

fn run(instructions: &Vec<Instruction>, tape: &mut [u8; 30000], ptr: &mut usize) {
    for instruction in instructions {
        match instruction {
            Instruction::Right => {
                *ptr = (*ptr + 1) % 30000;
            }
            Instruction::Left => {
                *ptr = (*ptr - 1) % 30000;
            }
            Instruction::Increment => {
                tape[*ptr] = tape[*ptr].wrapping_add(1);
            }
            Instruction::Decrement => {
                tape[*ptr] = tape[*ptr].wrapping_sub(1);
            }
            Instruction::Write => {
                print!("{}", tape[*ptr] as char);
                stdout().flush().ok().expect("Could not flush stdout");
            }
            Instruction::Read => {
                let input: Option<u8> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok());
                match input {
                    Some(new) => tape[*ptr] = new,
                    None => (),
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
