use clap::Parser;

// TODO
// -v

#[cfg(test)]
mod tests;

#[derive(Parser)]
struct Cli {
    /// Use RFC 2822 date and time output format.
    #[clap(short = 'R', action)]
    is_rfc_2822: bool,

    /// Display the date in UTC (Coordinated Universal) time.
    #[clap(short = 'u', action)]
    is_utc: bool,
}

#[derive(Debug, PartialEq)]
enum CustomError {}

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();

    let result = process_date_handler(args.is_rfc_2822, args.is_utc);
    match result {
        Ok(result) => println!("{}", result),
        Err(error) => eprintln!("Error: {}", parse_custom_error(error)),
    }

    Ok(())
}

fn parse_custom_error(error: CustomError) -> String {
    match error {}
}

fn process_date_handler(is_rfc_2822: bool, is_utc: bool) -> Result<String, CustomError> {
    match is_utc {
        true => process_date::<chrono::Utc>(is_rfc_2822, chrono::offset::Utc::now()),
        false => process_date::<chrono::Local>(is_rfc_2822, chrono::offset::Local::now()),
    }
}

fn process_date<T: chrono::TimeZone>(
    is_rfc_2822: bool,
    now: chrono::DateTime<T>,
) -> Result<String, CustomError>
where
    T::Offset: std::fmt::Display,
{
    if is_rfc_2822 {
        return Ok(now.to_rfc2822());
    }

    let format_specifier = "%a %b %e %H:%M:%S %Z %G";
    let date = now.format(format_specifier);

    Ok(date.to_string())
}
