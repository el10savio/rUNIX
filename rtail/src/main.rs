extern crate atty;

use clap::Parser;
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str), multiple = true)]
    paths: Vec<PathBuf>,

    /// Number of lines to print
    #[clap(short = 'n', value_parser, default_value_t = 10)]
    lines: usize,

    /// Number of bytes to print
    #[clap(short = 'c', conflicts_with = "lines")]
    bytes: Option<usize>,

    /// Suppress printing of headers
    #[clap(short = 'q', action)]
    suppress_headers: bool,
}

#[derive(Debug, PartialEq)]
enum CustomError {
    ErrNoLines,
    ErrNoBytes,
    ErrNoInput,
    ErrInvalidPath,
}

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let paths = args.paths;
    let max_lines = args.lines;
    let suppress_headers = args.suppress_headers;

    let result = match args.bytes {
        None => process_input(paths, max_lines, 0, false, suppress_headers),
        Some(max_bytes) => process_input(paths, max_lines, max_bytes, true, suppress_headers),
    };

    match result {
        Ok(result) => println!("{}", result),
        Err(error) => eprintln!("Error: {}", parse_custom_error(error)),
    }

    Ok(())
}

fn parse_custom_error(error: CustomError) -> String {
    match error {
        CustomError::ErrNoLines => "zero lines provided".to_string(),
        CustomError::ErrNoBytes => "zero bytes provided".to_string(),
        CustomError::ErrNoInput => "no input provided".to_string(),
        CustomError::ErrInvalidPath => "invalid filepath provided".to_string(),
    }
}

fn process_input(
    paths: Vec<PathBuf>,
    max_lines: usize,
    max_bytes: usize,
    is_bytes: bool,
    supress_headers: bool,
) -> Result<String, CustomError> {
    if paths.len() == 0 {
        return process_stdin(max_lines, max_bytes, is_bytes);
    }

    let multi_file = paths.len() > 1;
    let mut result: Vec<String> = vec![];

    for path in paths {
        if multi_file && !supress_headers {
            let file_header = get_file_header(&path);
            result.push(file_header);
        }

        let content = process_file(path, max_lines, max_bytes, is_bytes)?;
        if content != "" {
            result.push(content);
        }
    }

    Ok(result.join("\n"))
}

fn process_stdin(
    max_lines: usize,
    max_bytes: usize,
    is_bytes: bool,
) -> Result<String, CustomError> {
    if atty::is(atty::Stream::Stdin) {
        return Err(CustomError::ErrNoInput);
    }

    let reader = BufReader::new(io::stdin());

    match is_bytes {
        true => get_bytes::<std::io::Stdin>(reader, max_bytes),
        false => get_lines::<std::io::Stdin>(reader, max_lines),
    }
}

fn get_file_header(path: &PathBuf) -> String {
    let filename = Path::new(path).file_name().unwrap().to_str();
    format!("==> {} <==", filename.unwrap()).to_string()
}

fn process_file(
    path: PathBuf,
    max_lines: usize,
    max_bytes: usize,
    is_bytes: bool,
) -> Result<String, CustomError> {
    let file = open_file(&path)?;
    let reader = BufReader::new(file);
    match is_bytes {
        true => get_bytes::<std::fs::File>(reader, max_bytes),
        false => get_lines::<std::fs::File>(reader, max_lines),
    }
}

fn open_file(path: &PathBuf) -> Result<std::fs::File, CustomError> {
    match std::fs::File::open(path) {
        Err(_) => return Err(CustomError::ErrInvalidPath),
        Ok(file) => return Ok(file),
    }
}

fn get_lines<T: std::io::Read>(
    reader: BufReader<T>,
    max_lines: usize,
) -> Result<String, CustomError> {
    if max_lines == 0 {
        return Err(CustomError::ErrNoLines);
    }

    // TODO: Convert To Functional

    let mut result = vec![];
    let lines: Vec<_> = reader.lines().collect();

    let mut partition = 0;
    let count = lines.len();

    if max_lines < count {
        partition = count - max_lines;
    }

    lines.into_iter().enumerate().for_each(|(index, line)| {
        if index >= partition {
            result.push(line.unwrap());
        }
    });

    Ok(result.join("\n"))
}

fn get_bytes<T: std::io::Read>(
    reader: BufReader<T>,
    max_bytes: usize,
) -> Result<String, CustomError> {
    if max_bytes == 0 {
        return Err(CustomError::ErrNoBytes);
    }

    // TODO: Convert To Functional

    let mut result = vec![];
    let bytes: Vec<_> = reader.bytes().collect();

    let mut partition = 0;
    let count = bytes.len();

    if max_bytes < count {
        partition = count - max_bytes;
    }

    bytes.into_iter().enumerate().for_each(|(index, byte)| {
        if index >= partition {
            result.push(byte.unwrap());
        }
    });

    let mut result_string = String::from_utf8(result).expect("invalid UTF-8");
    result_string.pop();

    Ok(result_string)
}
