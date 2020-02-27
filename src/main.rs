use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_file = &args[1];

    let contents = fs::read_to_string(source_file)
        .expect("Something went wrong reading the file");
    
    
    println!("Source file: {}", source_file);
    println!("File contents:\n{}", contents);
}
