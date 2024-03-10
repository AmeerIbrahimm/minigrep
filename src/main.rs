use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_name = &args[2];
    println!("searching for string: {query}");
    println!("in a file: {}", file_name)
}
