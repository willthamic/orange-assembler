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
    raw: &str,
    label: Option<&str>,
    instruction: Option<Instruction>,
    comment: Option<&str>,
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

    let mut ilines = Vec::<ILine>::new();

    for line in lines {
        match process_line(line) {
            Ok(Some(x)) => ilines.push(x),
            Err(x) => eprintln!("{}", x),
            Ok(None) => (),
        }
    }

    for line in ilines {
        match line.label {
            Some(x) => print!("[{}]",x),
            None    => print!("[.]"),
        }
        // print!("[{:?}]", inst.opcode);
        match line.comment {
            Some(x) => print!("[{}]",x),
            None    => print!("[.]"),
        }
        println!();
    }

    Ok(())
}

fn process_line(raw: &'static str) -> Result<Option<ILine>, &'static str> {
    let line = raw.trim();
    let comment_pos = line.find(';');
    let (line, comment) = match comment_pos {
        Some(x) => (&line[..x], Some(&line[x..])),
        None    => (&line[..], None),
    };
    line = line.trim();

    let label_pos = line.find(':');
    let (line, label) = match label_pos {
        Some(x) => (&line[x..], Some(&line[..x])),
        None    => (&line[..], None)
    };
    line = line.trim();

    let instruction = match process_instruction(line) {
        Ok(x) => x,
        Err(x) => return Err(x),
    };

    Ok(Some(
        ILine{
            raw,
            label,
            instruction,
            comment,
        }
    ))
    
}

fn process_instruction(inst: &str) -> Result<Option<Instruction>, &'static str> {
    if inst.is_empty() {
        Ok(None)
    } else {
        let inst_pos = inst.find(' ');
        let opcode = match inst_pos {
            Some(x) => &inst[..x],
            None => return Err("invalid")
        };
        let opcode = match Opcode::from_str(opcode) {
            Ok(x) => x,
            Err(x) => return Err(&format!("{}", x)[..]),
        };
        Ok(Some(
            Instruction {
                opcode: opcode,
                r1: None,
                r2: None,
                r3: None,
                c1: None,
                c2: None,
                c3: None,
        }))
    }
}