use crate::{commands::utils::read_file, interpreter, parser::parse, tape, tokenizer::tokenize};
use crossterm::cursor;
use log::{debug, error};
use rustyline::{error::ReadlineError, DefaultEditor};

enum ReplCommand {
    Exit,
    Tape(Option<usize>),
    Invalid,
    Code(String),
}

impl ReplCommand {
    fn from_string(command: String) -> ReplCommand {
        let cmd = command.trim();

        if cmd == "exit" {
            ReplCommand::Exit
        } else if cmd == "tape" {
            ReplCommand::Tape(None)
        } else if cmd.starts_with("tape") {
            let splits = cmd.split_whitespace().collect::<Vec<&str>>();
            if splits.len() != 2 {
                error!("Invalid syntax: tape <index>");
                return ReplCommand::Invalid;
            }

            match splits[1].parse::<usize>() {
                Ok(index) => {
                    if index > 29999 {
                        error!("Index out of bounds: Length of tape is 30000");
                        ReplCommand::Invalid
                    } else {
                        ReplCommand::Tape(Some(index))
                    }
                }
                Err(_) => {
                    error!("Invalid index: Index must be a number");
                    ReplCommand::Invalid
                }
            }
        } else if cmd.starts_with("run") {
            let splits = cmd.split_whitespace().collect::<Vec<&str>>();
            if splits.len() != 2 {
                error!("Invalid syntax: run <file>");
                return ReplCommand::Invalid;
            }

            let source = read_file(splits[1]);

            ReplCommand::Code(source)
        } else {
            ReplCommand::Code(cmd.to_string())
        }
    }
}

pub fn repl() {
    println!("REPL Help:");
    println!("  - Type \"exit\" to exit the REPL");
    println!("  - Type \"tape\" to print the current state around the current index");
    println!("  - Type \"tape <index>\" to print the current state around the specified index");
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

        let readline = rl.readline(&format!("current index: {}> ", ptr));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();

                match ReplCommand::from_string(line) {
                    ReplCommand::Exit => {
                        println!("Exiting REPL");
                        break;
                    }
                    ReplCommand::Tape(index) => tape::print_tape(
                        tape,
                        match index {
                            Some(i) => i,
                            None => ptr,
                        },
                    ),
                    ReplCommand::Code(code) => {
                        let tokens = tokenize(code);
                        let instructions = parse(tokens);

                        interpreter::run(&instructions, &mut tape, &mut ptr);
                    }
                    ReplCommand::Invalid => {}
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
