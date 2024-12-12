use log::{debug, error};
use std::{fs, process};

/// Read a file into a string
pub fn read_file(file: &str) -> String {
    debug!("Attempting to read file {file}");

    let source = fs::read_to_string(file).unwrap_or_else(|err| {
        error!("Couldn't read {file}: {err}");
        process::exit(1);
    });

    debug!("Read file {file}");
    source
}
