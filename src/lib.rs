use std::fs;
pub fn validate_file_path(path: &Option<String>) -> Result<String, String> {
    let file_path = match path {
        Some(val) => val,
        None => return Err("File path missing".to_string()),
    };
    let read_res = fs::read_to_string(file_path);
    match read_res {
        Ok(val) => Ok(val),
        Err(_error) => Err("Failed to read file".to_string()),
    }
}
pub fn get_query_containing_lines(query: &String, file_contents: &String) -> Vec<String> {
    let mut matched_lines: Vec<String> = Vec::new();
    for line in file_contents.lines() {
        if line.contains(query) {
            println!("{}", line);
            matched_lines.push(String::from(line));
        }
    }
    matched_lines
}
