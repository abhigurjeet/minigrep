use minigrep;
use std::env;
fn main() {
    let params: Vec<String> = env::args().collect();
    let substring: Option<String> = params.get(1).cloned();
    let file_path: Option<String> = params.get(2).cloned();
    let mut query: String = String::new();
    match substring {
        Some(value) => {
            query.push_str(&value);
        }
        None => {
            println!("Error: Substring missing");
        }
    }
    match minigrep::validate_file_path(&file_path) {
        Ok(val) => {
            minigrep::get_query_containing_lines(&query, &val);
        }
        Err(error) => {
            println!("Error {}", error);
        }
    }
}
