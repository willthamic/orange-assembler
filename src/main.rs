use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Source file: {}", config.source_filename);
    println!("Output file: {}", config.output_filename);
    
    if let Err(e) = run(config) {
        println!("Application Error: {}", e);

        process::exit(1);
    }
    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.source_filename)?;

    println!("File contents:\n{}", contents);

    Ok(())
}

struct Config {
    source_filename: String,
    output_filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let source_filename = args[1].clone();
        let output_filename = "source.bin".to_string();

        Ok(Config { source_filename, output_filename })
    }
}