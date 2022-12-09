use clap::Parser;

// TODO: Add stdin support
// TODO: Add env variables support

#[cfg(test)]
mod tests;

#[derive(Parser)]
struct Cli {
    /// Values to print
    #[clap(required = true, multiple = true)]
    values: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CustomError {
    EmptyValues,
}

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();

    let result = process_echo(args.values);
    match result {
        Ok(result) => println!("{}", result),
        Err(error) => eprintln!("Error: {}", parse_custom_error(error)),
    }

    Ok(())
}

fn parse_custom_error(error: CustomError) -> String {
    match error {
        CustomError::EmptyValues => "empty values provided".to_string(),
    }
}

fn process_echo(values: Vec<String>) -> Result<String, CustomError> {
    if values.is_empty() {
        return Err(CustomError::EmptyValues);
    }
    Ok(values.join("\n"))
}
