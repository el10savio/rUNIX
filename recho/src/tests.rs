use super::*;
use test_case::test_case;

#[test_case(vec!["test".to_string()], "test".to_string(), None)]
#[test_case(vec!["test".to_string(), "test2".to_string()], "test\ntest2".to_string(), None)]
#[test_case(vec![], "".to_string(), Some(CustomError::EmptyValues))]
fn test_process_echo(
    values: Vec<String>,
    expected_result: String,
    expected_error: Option<CustomError>,
) {
    match expected_error {
        None => assert_eq!(
            expected_result,
            process_echo(values).unwrap()
        ),
        Some(expected_error) => assert_eq!(
            Err(expected_error),
            process_echo(values)
        ),
    }
}
