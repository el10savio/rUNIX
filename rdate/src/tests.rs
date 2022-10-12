// use super::*;
// use test_case::test_case;

// #[test_case(vec![std::path::PathBuf::from("")],  false, false, "".to_string(), Some(CustomError::ErrInvalidPath); "empty path")]
// fn test_process_date(
//     paths: Vec<std::path::PathBuf>,
//     line_numbers: bool,
//     non_blank_line_numbers: bool,
//     expected_result: String,
//     expected_error: Option<CustomError>,
// ) {
//     match expected_error {
//         None => assert_eq!(expected_result, process_date(paths, line_numbers, non_blank_line_numbers).unwrap()),
//         Some(expected_error) => assert_eq!(Err(expected_error), process_date(paths, line_numbers, non_blank_line_numbers)),
//     }
// }
