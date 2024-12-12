use crate::{commands::utils::read_file, interpreter, parser::parse, tape, tokenizer::tokenize};
use crossterm::cursor;
use log::{debug, error};
use rustyline::{error::ReadlineError, DefaultEditor};

pub fn repl() {
    println!("REPL Help:");
    println!("  - Type \"exit\" to exit the REPL");
    println!(
        "  - Type \"tape\" to print the current state of the tape around the current pointer index"
    );
    println!("  - Type \"tape <index>\" to print the value around the specified index");
    println!("  - Type \"run <file>\" to run brainfuck code from a file");
    println!("  - If you don't use any of these special keywords, the REPL will just function as a normal brainfuck interpreter");

    let mut tape: [u8; 30000] = [0; 30000];
    let mut ptr = 0;

    let mut rl = DefaultEditor::new().expect("Could not create editor");

    loop {
        let (x, _) = cursor::position().expect("Could not get cursor position");

        if x != 0 {
            println!();
        }

        let readline = rl.readline(format!("index: {}> ", ptr).as_str());
        match readline {
            Ok(line) => {
                if line == "exit" {
                    println!("Exiting REPL");
                    break;
                } else if line == "tape" {
                    tape::print_tape(tape, ptr);
                } else if line.starts_with("tape") {
                    match line[5..].trim().parse::<usize>() {
                        Ok(index) => {
                            if index > 29999 {
                                error!("Index out of bounds: Length of tape is 30000");
                            } else {
                                tape::print_tape(tape, index);
                            }
                        }
                        Err(_) => {
                            error!("Invalid index: Index must be a number");
                        }
                    }
                } else if line.starts_with("run") {
                    let path = line[4..].trim();
                    let source = read_file(path);

                    let tokens = tokenize(source);
                    let instructions = parse(tokens);

                    interpreter::run(&instructions, &mut tape, &mut ptr);
                } else {
                    let tokens = tokenize(line);
                    let instructions = parse(tokens);
                    interpreter::run(&instructions, &mut tape, &mut ptr);
                }
            }

            Err(ReadlineError::Interrupted) => {
                debug!("Got CTRL-C, exiting REPL");
                println!("Exiting REPL");
                break;
            }

            Err(ReadlineError::Eof) => {
                debug!("Got CTRL-D, exiting REPL");
                println!("Exiting REPL");
                break;
            }

            Err(err) => {
                println!("Got an error, exiting REPL");
                debug!("Error: {:?}", err);
                break;
            }
        }
    }
}
