use super::*;
use test_case::test_case;

// TODO
// Use -n count in tests and error out tests
// when -n is unbounded (not set)

#[test_case("".to_string(), "".to_string(), "".to_string(), Some(CustomError::EmptyHostname); "empty hostname")]
#[test_case("localhost".to_string(), "127.0.0.1".to_string(), "PING localhost (127.0.0.1): 56 data bytes".to_string(), None; "localhost")]
fn test_process_ping_header(
    hostname: String,
    address: String,
    expected_result: String,
    expected_error: Option<CustomError>,
) {
    match expected_error {
        None => assert_eq!(
            expected_result,
            process_ping_header(&hostname, &address).unwrap()
        ),
        Some(expected_error) => assert_eq!(
            Err(expected_error),
            process_ping_header(&hostname, &address)
        ),
    }
}

#[test_case("".to_string(), vec![], "".to_string(), Some(CustomError::EmptyHostname); "empty hostname")]
#[test_case("localhost".to_string(), vec![], "--- localhost ping statistics ---\n".to_string(), None; "localhost")]
fn test_process_ping_footer(
    hostname: String,
    durations: Vec<std::time::Duration>,
    expected_result: String,
    expected_error: Option<CustomError>,
) {
    match expected_error {
        None => assert_eq!(
            expected_result,
            process_ping_footer(&hostname, durations).unwrap()
        ),
        Some(expected_error) => assert_eq!(
            Err(expected_error),
            process_ping_footer(&hostname, durations)
        ),
    }
}
