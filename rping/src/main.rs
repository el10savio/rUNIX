use clap::Parser;
use dns_lookup::lookup_host;

mod ping;

#[cfg(test)]
mod tests;

#[derive(Parser)]
struct Cli {
    /// Hostname to ping
    #[clap(required = true)]
    hostname: String,

    /// Count of ping requests
    #[clap(short, default_value = "0")]
    count: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CustomError {
    EmptyHostname,
    InvalidHostname,
    EmptyAddress,
    EmptyIPAddressList,
    PingerCreationFailed,
}

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();

    let result = process_ping(args.hostname, args.count);
    match result {
        Ok(_) => (),
        Err(error) => eprintln!("Error: {}", parse_custom_error(error)),
    }

    Ok(())
}

fn parse_custom_error(error: CustomError) -> String {
    match error {
        CustomError::EmptyHostname => "empty hostname provided".to_string(),
        CustomError::InvalidHostname => "invalid hostname provided".to_string(),
        CustomError::EmptyAddress => "empty address provided".to_string(),
        CustomError::EmptyIPAddressList => "empty ip address list received".to_string(),
        CustomError::PingerCreationFailed => "failed to create pinger".to_string(),
    }
}

fn process_ping(hostname: String, count: u16) -> Result<(), CustomError> {
    // Check For Empty Hostname
    if hostname.is_empty() {
        return Err(CustomError::EmptyHostname);
    }

    // DNS Resolve Hostname
    let address = process_dns_hostname(&hostname)?;

    // Generate Ping Header
    let header = process_ping_header(&hostname, &address)?;
    println!("{}", header);

    // Perform Ping Requests
    let durations = process_ping_requests(&address, count)?;
    println!();

    // Generate Ping Footer
    let footer = process_ping_footer(&hostname, durations)?;
    println!("{}", footer);

    Ok(())
}

fn process_ping_header(hostname: &String, address: &String) -> Result<String, CustomError> {
    // Check For Empty Hostname
    if hostname.is_empty() {
        return Err(CustomError::EmptyHostname);
    }

    // Check For Empty Address
    if address.is_empty() {
        return Err(CustomError::EmptyAddress);
    }

    // ICMP ECHO_REQUEST
    // Data Bytes Size
    let bytes = "56 data bytes";

    // Form Ping Header
    let header = format!("PING {} ({}): {}", hostname, address, bytes);

    Ok(header)
}

fn process_ping_requests(
    address: &String,
    count: u16,
) -> Result<Vec<std::time::Duration>, CustomError> {
    // Check For Empty Address
    if address.is_empty() {
        return Err(CustomError::EmptyAddress);
    }

    // Loop Ping Requests
    let durations = ping::transmit_packets(address.to_string(), count)?;

    Ok(durations)
}

fn process_ping_footer(
    hostname: &String,
    durations: Vec<std::time::Duration>,
) -> Result<String, CustomError> {
    // Check For Empty Hostname
    if hostname.is_empty() {
        return Err(CustomError::EmptyHostname);
    }

    // Form Ping Footer
    let footer = format!("--- {} ping statistics ---", hostname);

    // Form Ping Statistics
    let results = stats(durations)?;


    Ok(format!("{}\n{}", footer, results))
}

fn process_dns_hostname(hostname: &String) -> Result<String, CustomError> {
    // Check For Empty Hostname
    if hostname.is_empty() {
        return Err(CustomError::EmptyHostname);
    }

    // Get IP Addresses From Hostname
    let ip_address_list = match lookup_host(hostname) {
        Ok(ip_address_list) => ip_address_list,
        Err(_error) => return Err(CustomError::InvalidHostname),
    };

    // Check For Empty ip_address_list
    if ip_address_list.is_empty() {
        return Err(CustomError::EmptyIPAddressList);
    }

    // Get Only The Last Entry
    let address = ip_address_list.last().unwrap();

    Ok(address.to_string())
}

fn stats(durations: Vec<std::time::Duration>) -> Result<String, CustomError>{
    if durations.is_empty() {
        return Ok("".to_string());
    }

		// TODO: Implement stddev
    // round-trip min/avg/max/stddev = 26.404/29.191/32.247/2.123 ms

    let count = durations.len() as u32;
    let min = durations.iter().min().unwrap();
    let max = durations.iter().max().unwrap();
    let sum = durations.iter().sum::<std::time::Duration>();
    let avg: std::time::Duration = sum / count;

    let response = format!(
        "round-trip min/avg/max = {:?}/{:?}/{:?}",
        min, avg, max
    );

		Ok(response)
}
