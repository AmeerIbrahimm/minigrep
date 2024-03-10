use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_name = &args[2];
    println!("searching for string: {query}");
    println!("in a file: {}", file_name);

    let contents = fs::read_to_string(file_name).expect("not able to open a file");

    println!("file contents:\n{contents}")
}
