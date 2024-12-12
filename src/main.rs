mod commands;
pub mod interpreter;
pub mod parser;
pub mod tape;
pub mod tokenizer;

use clap::{Parser, Subcommand};
use env_logger::Builder;
use log::{debug, info, LevelFilter};
use std::io::Write;

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
            let warn_style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "[{warn_style}{}{warn_style:#}] {}",
                record.level(),
                record.args()
            )
        })
        .filter(
            Some("brainfuck"),
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
            commands::run(file);
        }

        Commands::Check { file } => {
            debug!("Running command \"check\"");
            commands::check(file);
        }

        Commands::Minify {
            file,
            output,
            print,
        } => {
            debug!("Running command \"minify\"");
            commands::minify(file, output.clone(), print);
        }

        Commands::Repl {} => {
            debug!("Running command \"repl\"");
            commands::repl();
        }

        Commands::Eval { code } => {
            debug!("Running command \"eval\"");
            commands::eval(code);
        }
    }
}
