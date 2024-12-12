use crate::{
    commands::utils::read_file,
    tokenizer::{source_from_tokens, tokenize},
};
use humansize::{format_size, WINDOWS};
use log::{debug, info};
use std::{fs, path::Path};

pub fn minify(file: &str, output: Option<String>, print: &bool) {
    let output_file: String;
    let source = read_file(file);

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

    fs::write(&output_file, minified.clone()).expect("Error while writing to file {output_file}");

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
