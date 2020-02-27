use std::error::Error;
use std::fs;
use std::path::PathBuf;

extern crate strum;
#[macro_use]
extern crate strum_macros;
use std::str::FromStr;

pub struct Config {
    pub source_path: PathBuf,
    pub output_path: PathBuf,
}

struct ILine {
    raw: String,
    label: Option<String>,
    instruction: Option<Instruction>,
    comment: Option<String>,
}

struct Instruction {
    opcode: Opcode,
    r1: Option<usize>,
    r2: Option<usize>,
    r3: Option<usize>,
    c1: Option<usize>,
    c2: Option<usize>,
    c3: Option<usize>,
}

#[derive(EnumString)]
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
        match inst.label {
            Some(x) => print!("[{}]",x),
            None    => print!("[.]"),
        }
        // print!("[{:?}]", inst.opcode);
        match inst.comment {
            Some(x) => print!("[{}]",x),
            None    => print!("[.]"),
        }
        println!();
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
        let (comment, line_trimmed) = match comment_pos {
            Some(x) => (
                Some(line[(x+1)..].trim().to_string()), 
                &line_trimmed[..x]
            ),
            None => (None, line_trimmed),
        };
        let line_trimmed = line_trimmed.trim();

        let label_pos = line_trimmed.find(':');
        let (label, line_trimmed) = match label_pos {
            Some(x) => (Some(line[..x].to_string()), &line_trimmed[x..]),
            None    => (None, line_trimmed),
        };
        let line_trimmed = line_trimmed.trim();

        let inst_pos = line_trimmed.find(' ');
        let (inst, line_trimmed) = match inst_pos {
            Some(x) => (Some(line[..x].to_string()), &line_trimmed[x..]),
            None => (None, line_trimmed),
        };
        let line_trimmed = line_trimmed.trim();

        let inst = Opcode::from_str(inst);

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

fn parse_instruction(inst: String) -> (opcode, usize, usize)