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

struct ILine<'a> {
    raw: &'a str,
    label: Option<&'a str>,
    instruction: Option<Instruction>,
    comment: Option<&'a str>,
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

enum InstructionFormat {
    OP,
    OP_RA_RB_RC,
    OP_RA_RB_C2,
    OP_RA_C1,
    OP_RA_RC,
    OP_RB_RC_C3,
    OP_RA_RB_RC_C3,
    OP_RA_RB_N,
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

fn process_line<'a> (raw: &'a str) -> Result<Option<ILine>, &'static str> {
    let line = raw.trim();
    let comment_pos = line.find(';');
    let (line, comment) = match comment_pos {
        Some(x) => (&line[..x], Some(&line[x..])),
        None    => (&line[..], None),
    };
    let line = line.trim();

    let label_pos = line.find(':');
    let (line, label) = match label_pos {
        Some(x) => (&line[x..], Some(&line[..x])),
        None    => (&line[..], None)
    };
    let line = line.trim();

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
        let (opcode, inst) = match inst_pos {
            Some(x) => (&inst[..x], &inst[x..]),
            None => return Err("invalid")
        };
        let opcode = match Opcode::from_str(opcode) {
            Ok(x) => x,
            Err(_) => return Err("Opcode not found"),
        };
        let inst = match opcode {
            Opcode::LA => process_OP_RA_RB_C2(inst, opcode),
        };
        match inst {
            Ok(x) => Ok(Some(x)),
            Err(x) => Err(x)
        }
    }
}

    // OP,
    // OP_RA_RB_RC,
    // OP_RA_RB_C2,
    // OP_RA_C1,
    // OP_RA_RC,
    // OP_RB_RC_C3,
    // OP_RA_RB_RC_C3,
    // OP_RA_RB_N,

fn process_OP (inst: &str, opcode: Opcode) -> Result<Instruction, &'static str> {
    match inst.is_empty() {
        True => {
            Ok(Instruction {
                opcode: opcode,
                r1: None,
                r2: None,
                r3: None,
                c1: None,
                c2: None,
                c3: None,
            })
        }
    }
    
}

fn process_OP_RA_RB_C2 (inst: &str, opcode: Opcode) -> Result<Instruction, &'static str> {
    
    Ok(Instruction {
            opcode: opcode,
            r1: None,
            r2: None,
            r3: None,
            c1: None,
            c2: None,
            c3: None,
    })
}