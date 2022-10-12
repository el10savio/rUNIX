use super::*;
use test_case::test_case;

#[test_case("src/test_data/test.txt".to_string(), 2, "Line 1\nLine 2".to_string(), None; "base case")]
#[test_case("src/test_data/test.txt".to_string(), 200, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5".to_string(), None; "lines greater than file")]
#[test_case("src/test_data/test_empty.txt".to_string(), 2, "".to_string(), None; "empty file")]
#[test_case("src/test_data/test.txt".to_string(), 0, "".to_string(), Some(CustomError::ErrNoLines); "zero lines")]
fn test_get_lines(
    path: String,
    max_lines: usize,
    expected_result: String,
    expected_error: Option<CustomError>,
) {
    let file = std::fs::File::open(path).unwrap();
    let content = BufReader::new(file);

    match expected_error {
        None => assert_eq!(expected_result, get_lines(content, max_lines).unwrap()),
        Some(expected_error) => assert_eq!(Err(expected_error), get_lines(content, max_lines)),
    }
}

#[test_case("src/test_data/test.txt".to_string(), 2, "Li".to_string(), None; "base case")]
#[test_case("src/test_data/test.txt".to_string(), 5, "Line ".to_string(), None; "base case 2")]
#[test_case("src/test_data/test.txt".to_string(), 34, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5".to_string(), None; "all bytes of file")]
#[test_case("src/test_data/test.txt".to_string(), 200000, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\n".to_string(), None; "bytes greater than file")]
#[test_case("src/test_data/test_empty.txt".to_string(), 2, "".to_string(), None; "empty file")]
#[test_case("src/test_data/test.txt".to_string(), 0, "".to_string(), Some(CustomError::ErrNoBytes); "zero bytes")]
fn test_get_bytes(
    path: String,
    max_bytes: usize,
    expected_result: String,
    expected_error: Option<CustomError>,
) {
    let file = std::fs::File::open(path).unwrap();
    let content = BufReader::new(file);

    match expected_error {
        None => assert_eq!(expected_result, get_bytes(content, max_bytes).unwrap()),
        Some(expected_error) => assert_eq!(Err(expected_error), get_bytes(content, max_bytes)),
    }
}

#[test_case(vec![std::path::PathBuf::from("")], 2, 0, false, "".to_string(), Some(CustomError::ErrInvalidPath); "empty path")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test_invalid.txt")], 2, 0, false, "".to_string(), Some(CustomError::ErrInvalidPath); "file not present")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt"),std::path::PathBuf::from("src/test_data/test_2.txt")], 2, 0, false, "==> test.txt <==\nLine 1\nLine 2\n==> test_2.txt <==\nLine A\nLine B".to_string(), None; "line - base case")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt"),std::path::PathBuf::from("src/test_data/test_empty.txt")], 2, 0, false, "==> test.txt <==\nLine 1\nLine 2\n==> test_empty.txt <==".to_string(), None; "line - with empty file")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt")], 2, 0, false, "Line 1\nLine 2".to_string(), None; "line - one file")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt")], 0, 0, false, "".to_string(), Some(CustomError::ErrNoLines); "line - zero lines")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt"),std::path::PathBuf::from("src/test_data/test_2.txt")], 0, 2, true, "==> test.txt <==\nLi\n==> test_2.txt <==\nLi".to_string(), None; "bytes - base case")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt"),std::path::PathBuf::from("src/test_data/test_empty.txt")], 0, 2, true, "==> test.txt <==\nLi\n==> test_empty.txt <==".to_string(), None; "bytes - with empty file")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt")], 0, 2, true, "Li".to_string(), None; "bytes - one file")]
#[test_case(vec![std::path::PathBuf::from("src/test_data/test.txt")], 0, 0, true, "".to_string(), Some(CustomError::ErrNoBytes); "bytes - zero bytes")]
fn test_process_input(
    paths: Vec<std::path::PathBuf>,
    max_lines: usize,
    max_bytes: usize,
    is_bytes: bool,
    expected_result: String,
    expected_error: Option<CustomError>,
) {
    match expected_error {
        None => assert_eq!(
            expected_result,
            process_input(paths, max_lines, max_bytes, is_bytes).unwrap()
        ),
        Some(expected_error) => assert_eq!(
            Err(expected_error),
            process_input(paths, max_lines, max_bytes, is_bytes)
        ),
    }
}
