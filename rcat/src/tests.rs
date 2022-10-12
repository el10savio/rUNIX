use super::*;
use test_case::test_case;

#[test_case(vec!["src/test_data/test.txt".to_string()], false, false, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5".to_string(), None; "base case")]
#[test_case(vec!["src/test_data/test_empty.txt".to_string()], false, false, "".to_string(), None; "empty file")]
#[test_case(vec!["src/test_data/test.txt".to_string()], true, false, "\t1 Line 1\n\t2 Line 2\n\t3 Line 3\n\t4 Line 4\n\t5 Line 5".to_string(), None; "base case + line numbers")]
#[test_case(vec!["src/test_data/test_empty.txt".to_string()], true, false, "".to_string(), None; "empty file + line numbers")]
#[test_case(vec!["src/test_data/test.txt".to_string()], false, true, "\t1 Line 1\n\t2 Line 2\n\t3 Line 3\n\t4 Line 4\n\t5 Line 5".to_string(), None; "base case + non blank line numbers")]
#[test_case(vec!["src/test_data/test_empty.txt".to_string()], false, true, "".to_string(), None; "empty file + non blank line numbers")]
#[test_case(vec!["src/test_data/test.txt".to_string()], true, true, "\t1 Line 1\n\t2 Line 2\n\t3 Line 3\n\t4 Line 4\n\t5 Line 5".to_string(), None; "base case + line numbers + non blank line numbers")]
#[test_case(vec!["src/test_data/test_empty.txt".to_string()], true, true, "".to_string(), None; "empty file + line numbers + non blank line numbers")]
fn test_get_lines(
    paths: Vec<String>,
    line_numbers: bool,
    non_blank_line_numbers: bool,
    expected_result: String,
    expected_error: Option<CustomError>,
) {
    let mut readers = vec![];

    for path in paths {
        let file = std::fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        readers.push(reader)
    }

    match expected_error {
        None => assert_eq!(expected_result, get_lines(readers, line_numbers, non_blank_line_numbers).unwrap()),
        Some(expected_error) => {
            assert_eq!(Err(expected_error), get_lines(readers, line_numbers, non_blank_line_numbers))
        }
    }
}

#[test_case(vec![std::path::PathBuf::from("")],  false, false, "".to_string(), Some(CustomError::ErrInvalidPath); "empty path")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test_invalid.txt")], false,  false, "".to_string(), Some(CustomError::ErrInvalidPath); "file not present")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt"),std::path::PathBuf::from("src/test_data/test_2.txt")], false,  false, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine A\nLine B\nLine C\nLine D\nLine E".to_string(), None; "multiple files")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt"),std::path::PathBuf::from("src/test_data/test_empty.txt")], false,  false, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5".to_string(), None; "multiple files with empty file")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt")],  false, false, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5".to_string(), None; "one file")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt"),std::path::PathBuf::from("src/test_data/test_2.txt")], true,  false, "\t1 Line 1\n\t2 Line 2\n\t3 Line 3\n\t4 Line 4\n\t5 Line 5\n\t6 Line A\n\t7 Line B\n\t8 Line C\n\t9 Line D\n\t10 Line E".to_string(), None; "multiple files + line numbers")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt"),std::path::PathBuf::from("src/test_data/test_empty.txt")], true,  false, "\t1 Line 1\n\t2 Line 2\n\t3 Line 3\n\t4 Line 4\n\t5 Line 5".to_string(), None; "multiple files with empty file + line numbers")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt")],  true, false, "\t1 Line 1\n\t2 Line 2\n\t3 Line 3\n\t4 Line 4\n\t5 Line 5".to_string(), None; "one file + line numbers")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt"),std::path::PathBuf::from("src/test_data/test_empty_content.txt"),std::path::PathBuf::from("src/test_data/test_2.txt")], false,  true, "\t1 Line 1\n\t2 Line 2\n\t3 Line 3\n\t4 Line 4\n\t5 Line 5\n\n\n\t6 Line A\n\t7 Line B\n\t8 Line C\n\t9 Line D\n\t10 Line E".to_string(), None; "multiple files with empty + non blank line numbers")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt"),std::path::PathBuf::from("src/test_data/test_empty.txt")], false,  true, "\t1 Line 1\n\t2 Line 2\n\t3 Line 3\n\t4 Line 4\n\t5 Line 5".to_string(), None; "multiple files with empty file + non blank line numbers")]
fn test_process_input(
    paths: Vec<std::path::PathBuf>,
    line_numbers: bool,
    non_blank_line_numbers: bool,
    expected_result: String,
    expected_error: Option<CustomError>,
) {
    match expected_error {
        None => assert_eq!(expected_result, process_input(paths, line_numbers, non_blank_line_numbers).unwrap()),
        Some(expected_error) => assert_eq!(Err(expected_error), process_input(paths, line_numbers, non_blank_line_numbers)),
    }
}
