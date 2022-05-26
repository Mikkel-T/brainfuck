use clap::Parser;
use std::fs;
use std::io::{stdout, Read, Write};
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The brainfuck program to run
    file: String,
}

fn main() {
    let args = Args::parse();

    let program = fs::read_to_string(&args.file).unwrap_or_else(|err| {
        println!("couldn't read {}: {}", &args.file, err);
        process::exit(1);
    });

    let mut tape: [u8; 30000] = [0; 30000];
    let mut ptr = 0;
    run(&program, &mut tape, &mut ptr)
}

fn run(program: &str, tape: &mut [u8; 30000], ptr: &mut usize) {
    let mut code: Vec<_> = program.chars().rev().enumerate().collect();

    while let Some((_, command)) = code.pop() {
        match command {
            '>' => {
                *ptr = (*ptr + 1) % 30000;
            }
            '<' => {
                *ptr = (*ptr - 1) % 30000;
            }
            '+' => {
                tape[*ptr] = tape[*ptr].wrapping_add(1);
            }
            '-' => {
                tape[*ptr] = tape[*ptr].wrapping_sub(1);
            }
            '.' => {
                print!("{}", tape[*ptr] as char);
                stdout().flush().ok().expect("Could not flush stdout");
            }
            ',' => {
                let input: Option<u8> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok());
                match input {
                    Some(new) => tape[*ptr] = new,
                    None => (),
                };
            }
            '[' => {
                let mut loop_stack = 1;
                let mut loop_code = String::new();
                while let Some((_, cmd)) = code.pop() {
                    if cmd == '[' {
                        loop_stack += 1;
                    } else if cmd == ']' {
                        loop_stack -= 1;
                    }

                    if cmd == ']' && loop_stack == 0 {
                        break;
                    }
                    loop_code.push(cmd);
                }
                while tape[*ptr] != 0 {
                    run(loop_code.as_str(), tape, ptr);
                }
            }
            _ => (),
        }
    }
}
