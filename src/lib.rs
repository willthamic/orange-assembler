use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub source_path: PathBuf,
    pub output_path: PathBuf,
}

struct ILine {
    raw: String,
    label: Option<String>,
    opcode: Opcode,
    comment: Option<String>,
}

enum Opcode {
    LD,
    LDR,
    ST,
    STR,
    LA,
    LAR,
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.source_path)?;
    let lines = contents.lines();

    let mut instructions = Vec::<ILine>::new();

    for line in lines {
        match process_line(line.to_string()) {
            Some(Ok(x)) => instructions.push(x),
            Some(Err(x)) => eprintln!("{}", x),
            None => (),
        }
    }

    for inst in instructions {
        println!("{}", inst.)
    }

    Ok(())
}

fn process_line(line: String) -> Option<Result<ILine, &'static str>> {
    let line = line.trim();
    let comment_pos = line.find(';');
    let line_trimmed = match comment_pos {
        Some(x) => &line[..x],
        None    => &line,
    };
    let empty = line_trimmed.is_empty();
    if !empty {
        let comment = match comment_pos {
            Some(x) => Some(line[x..].to_string()),
            None    => None,
        };
        let label_pos = line_trimmed.find(':');
        let label = match label_pos {
            Some(x) => Some(line[..x].to_string()),
            None    => None,
        };
        Some(Ok(ILine{
            raw: line.to_string(),
            label: label,
            opcode: Opcode::LD,
            comment: comment,
        }))
    } else {
        None
    }
}