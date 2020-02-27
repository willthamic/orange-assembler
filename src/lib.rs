use std::error::Error;
use std::fs;

pub struct Config {
    pub source_filename: String,
    pub output_filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let source_filename = args[1].clone();
        let output_filename = "source.bin".to_string();

        Ok(Config { source_filename, output_filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.source_filename)?;

    println!("File contents:\n{}", contents);

    Ok(())
}