pub mod interpreter;
pub mod parser;
pub mod tape;
pub mod tokenizer;

use clap::{Parser, Subcommand};
use crossterm::cursor;
use env_logger::Builder;
use humansize::{format_size, WINDOWS};
use log::{debug, error, info, LevelFilter};
use parser::parse;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process;
use tokenizer::{source_from_tokens, tokenize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Turn debugging information on
    #[clap(short, long)]
    debug: bool,
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

    /// Minifies a brainfuck program, removing all comments
    Minify {
        /// The brainfuck program to minify
        file: String,

        /// The name of the output file
        #[clap(short, long, name = "FILE")]
        output: Option<String>,

        /// Whether or not the output should be printed to the terminal
        #[clap(short, long)]
        print: bool,
    },

    /// Spins up a REPL instance where brainfuck code can be run while also checking the state of the tape
    Repl {},

    /// Evaluates a piece of brainfuck code straight from the terminal
    Eval { code: String },
}

fn main() {
    let args = Cli::parse();

    let mut builder = Builder::new();

    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {}",
                buf.default_styled_level(record.level()),
                record.args()
            )
        })
        .filter(
            None,
            if args.debug {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
        )
        .init();

    if args.debug {
        info!("Debug mode turned on");
    }

    match &args.command {
        Commands::Run { file } => {
            debug!("Running command \"run\"");
            let source = read_file(file.to_string());

            let tokens = tokenize(source);
            debug!("Parsing the tokens");
            let instructions = parse(tokens);
            debug!("Done parsing");

            let mut tape: [u8; 30000] = [0; 30000];
            let mut ptr = 0;
            debug!("Running the program");

            interpreter::run(&instructions, &mut tape, &mut ptr);
        }

        Commands::Check { file } => {
            debug!("Running command \"check\"");
            let source = read_file(file.to_string());

            println!("Checking the file {file}");

            let tokens = tokenize(source);

            debug!("Parsing the tokens");
            parse(tokens);
            debug!("Done parsing");

            println!("No issues found with the file {file}");
        }

        Commands::Minify {
            file,
            output,
            print,
        } => {
            debug!("Running command \"minify\"");
            let output_file: String;
            let source = read_file(file.to_string());

            debug!("Minifying the source");
            let tokens = tokenize(source.clone());
            let minified = source_from_tokens(tokens);
            debug!("Done minifying the source");

            match output {
                Some(name) => output_file = name.to_string(),
                None => {
                    let path = Path::new(&file);
                    let file_stem = path.file_stem().unwrap().to_str().unwrap();
                    let extension = path.extension().unwrap().to_str().unwrap();

                    output_file = format!("{file_stem}.min.{extension}");
                    debug!("No output file specified. Using {output_file}");
                }
            }

            debug!("Attempting to write to output file");

            fs::write(&output_file, minified.clone())
                .expect("Error while writing to file {output_file}");

            info!("Minified {file} -> {output_file}");

            let source_len = source.len();
            let minified_len = minified.len();

            info!(
                "{} -> {} ({}%)",
                format_size(source_len, WINDOWS),
                format_size(minified_len, WINDOWS),
                ((minified_len as f32 - source_len as f32) / source_len as f32) * 100.
            );

            if *print {
                info!("Minified code:");
                println!("{minified}");
            }
        }

        Commands::Repl {} => {
            debug!("Running command \"repl\"");
            println!("REPL Help:");
            println!("  - Type \"exit\" to exit the REPL");
            println!("  - Type \"tape\" to print the current state of the tape around the current pointer index");
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
                            let source = read_file(path.to_string());

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

        Commands::Eval { code } => {
            debug!("Running command \"eval\"");
            let tokens = tokenize(code.to_string());
            let instructions = parse(tokens);

            let mut tape: [u8; 30000] = [0; 30000];
            let mut ptr = 0;

            interpreter::run(&instructions, &mut tape, &mut ptr);
        }
    }
}

/// Read a file into a string
fn read_file(file: String) -> String {
    debug!("Attempting to read file {file}");

    let source = fs::read_to_string(&file).unwrap_or_else(|err| {
        error!("Couldn't read {file}: {err}");
        process::exit(1);
    });

    debug!("Read file {file}");
    source
}
