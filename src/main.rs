use std::env;
use std::fs;
fn main() {
    let params: Vec<String> = env::args().collect();
    let substring: Option<String> = params.get(1).cloned();
    let file_path: Option<String> = params.get(2).cloned();
    let mut query: String = String::new();
    let mut file_contents: String = String::new();
    match substring {
        Some(value) => {
            query.push_str(&value);
            println!("Substring value: {}", value);
        }
        None => {
            println!("Error: Substring missing");
        }
    }
    match file_path {
        Some(value) => {
            let res = fs::read_to_string(value);
            match res {
                Ok(val) => {
                    file_contents.push_str(&val);
                    for line in file_contents.lines() {
                        if line.contains(&query) {
                            println!("{}", line);
                        }
                    }
                }
                Err(error) => {
                    println!("Error:{}", error);
                }
            }
        }
        None => {
            println!("Error: File path missing");
        }
    }
}
// fn get_params() -> (Option<String>, Option<String>) {
//     let params: Vec<String> = env::args().collect();
//     let substr_option: Option<String> = params.get(1).cloned();
//     let file_path_option: Option<String> = params.get(2).cloned();
//     return (substr_option, file_path_option);
// }
