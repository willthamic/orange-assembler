use std::env;
use std::process;

use orange_assembler::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Source file: {}", config.source_filename);
    println!("Output file: {}", config.output_filename);
    
    if let Err(e) = orange_assembler::run(config) {
        println!("Application Error: {}", e);

        process::exit(1);
    }
}
