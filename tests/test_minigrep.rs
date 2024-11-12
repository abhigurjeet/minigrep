use minigrep::get_query_containing_lines;
use minigrep::validate_file_path;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*; // Only if validate_file_path is in the same module; otherwise, import properly
                  // use std::io;

    #[test]
    fn check_invalid_file_path() {
        let res = validate_file_path(&Some("non_existent_file.txt".to_string()));
        match res {
            Ok(_val) => {}
            Err(error) => {
                assert_eq!(error, "Failed to read file".to_string());
            }
        }
    }
    #[test]
    fn check_valid_file_path() {
        let res = validate_file_path(&Some("src/main.rs".to_string()));
        let file_contents = fs::read_to_string("src/main.rs").unwrap();
        match res {
            Ok(val) => {
                assert_eq!(val, file_contents);
            }
            Err(_error) => {}
        }
    }
    #[test]
    fn check_for_some_matched_lines() {
        let file_contents = fs::read_to_string("src/main.rs").unwrap();
        let res = get_query_containing_lines(&"let".to_string(), &file_contents);
        assert!(res.len() > 0);
    }
    #[test]
    fn check_for_no_matched_lines() {
        let file_contents = fs::read_to_string("src/main.rs").unwrap();
        let res = get_query_containing_lines(&"kidda".to_string(), &file_contents);
        assert!(res.len() == 0);
    }
}
