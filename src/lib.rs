use std::error::Error;
use std::fs;
use std::path::PathBuf;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod inst;
mod prog;

pub struct Config {
    pub source_path: PathBuf,
    pub output_path: PathBuf,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let source_path = PathBuf::from(args[1].clone());
        let output_path = source_path.with_extension("bin");

        Ok(Config { source_path, output_path })
    }
}

pub fn run<'a> (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.source_path)?;

    let ilines = prog::Prog::new("file", &contents).unwrap(); // TEMP FIX

    let encoded = ilines.encode().unwrap(); // TEMP FIX

    fs::write(config.output_path, encoded)
        .expect("Unable to write file");

    Ok(())
}

#[cfg(test)]
mod test;

// #[cfg(test)]
// mod inst;