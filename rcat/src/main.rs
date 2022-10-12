extern crate atty;

use clap::Parser;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[cfg(test)]
mod tests;

#[derive(Parser)]
struct Cli {
    /// The path(s) to the file(s) to read
    #[clap(parse(from_os_str), multiple = true)]
    paths: Vec<PathBuf>,

    /// Add line numbers
    #[clap(short = 'n', action)]
    line_numbers: bool,

    /// Add line numbers only to non blank lines
    #[clap(short = 'b', action)]
    non_blank_line_numbers: bool,
}

#[derive(Debug, PartialEq)]
enum CustomError {
    ErrNoInput,
    ErrInvalidPath,
}

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let paths = args.paths;
    let line_numbers = args.line_numbers;
    let non_blank_line_numbers = args.non_blank_line_numbers;

    let result = process_input(paths, line_numbers, non_blank_line_numbers);
    match result {
        Ok(result) => println!("{}", result),
        Err(error) => eprintln!("Error: {}", parse_custom_error(error)),
    }

    Ok(())
}

fn parse_custom_error(error: CustomError) -> String {
    match error {
        CustomError::ErrNoInput => "no input provided".to_string(),
        CustomError::ErrInvalidPath => "invalid filepath provided".to_string(),
    }
}

fn process_input(
    paths: Vec<PathBuf>,
    line_numbers: bool,
    non_blank_line_numbers: bool,
) -> Result<String, CustomError> {
    if paths.len() == 0 {
        return process_stdin(line_numbers, non_blank_line_numbers);
    }

    let mut result: Vec<String> = vec![];

    let content = process_files(paths, line_numbers, non_blank_line_numbers)?;
    if content != "" {
        result.push(content);
    }

    Ok(result.join("\n"))
}

fn process_stdin(line_numbers: bool, non_blank_line_numbers: bool) -> Result<String, CustomError> {
    if atty::is(atty::Stream::Stdin) {
        return Err(CustomError::ErrNoInput);
    }

    let mut readers = vec![];
    let reader = BufReader::new(io::stdin());
    readers.push(reader);

    get_lines::<std::io::Stdin>(readers, line_numbers, non_blank_line_numbers)
}

fn process_files(
    paths: Vec<PathBuf>,
    line_numbers: bool,
    non_blank_line_numbers: bool,
) -> Result<String, CustomError> {
    let mut readers = vec![];

    for path in paths {
        let file = open_file(&path)?;
        let reader = BufReader::new(file);
        readers.push(reader);
    }

    get_lines::<std::fs::File>(readers, line_numbers, non_blank_line_numbers)
}

fn open_file(path: &PathBuf) -> Result<std::fs::File, CustomError> {
    match std::fs::File::open(path) {
        Err(_) => return Err(CustomError::ErrInvalidPath),
        Ok(file) => return Ok(file),
    }
}

fn get_lines<T: std::io::Read>(
    readers: Vec<BufReader<T>>,
    line_numbers: bool,
    non_blank_line_numbers: bool,
) -> Result<String, CustomError> {
    let mut line_index: usize = 1;
    let mut result = vec![];

    for reader in readers {
        for line in reader.lines() {
            let mut line_content = line.unwrap();

						if non_blank_line_numbers && line_content == "" {
								result.push("".to_string());
								continue
						}
						
            if line_numbers || non_blank_line_numbers {
                let prefix = format!("\t{} ", &(line_index).to_string());
                line_content = prefix + &line_content;
                line_index += 1;
            }
						
            result.push(line_content);
        }
    }

    Ok(result.join("\n"))
}
