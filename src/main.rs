pub mod interpreter;
pub mod parser;
pub mod tokenizer;

use clap::{Parser, Subcommand};
use env_logger::Builder;
use humansize::{WINDOWS, format_size};
use log::{debug, error, info, LevelFilter};
use parser::parse;
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
    },
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
        Commands::Minify { file, output } => {
            debug!("Running command \"minify\"");
            let output_file: String;
            let source = read_file(file.to_string());

            debug!("Minifying the source");
            let minified = source_from_tokens(tokenize(source.clone()));
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
            println!("Minified {file} -> {output_file}");
            let source_len = source.len();
            let minified_len = minified.len();
            println!(
                "{} -> {} ({}%)",
                format_size(source_len, WINDOWS),
                format_size(minified_len, WINDOWS),
                ((minified_len as f32 - source_len as f32) / source_len as f32) * 100.
            );
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
