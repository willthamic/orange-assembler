use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::fmt;

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
    args: Arguments,
}

struct Arguments {
    ra: Option<usize>,
    rb: Option<usize>,
    rc: Option<usize>,
    c1: Option<usize>,
    c2: Option<usize>,
    c3: Option<usize>,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "{}{:?}{:?}{:?}{:?}{:?}{:?}", 
            self.opcode, 
            self.args.ra, 
            self.args.rb, 
            self.args.rc, 
            self.args.c1, 
            self.args.c2, 
            self.args.c3
        )
    }
}

#[derive(EnumString, Display, Debug, PartialEq)]
enum Opcode {
    // op
    NOP, 
    STOP,

    // op_ra_rb_rc
    ADD, 
    SUB, 
    AND, 
    OR,

    // op_ra_rb_c2
    ADDI,
    ANDI,
    ORI,
    LD,
    ST,
    LA,

    // op_ra_c1
    LDR,
    STR,
    LAR,
    
    // op_ra_rc
    NEG,
    NOT,

    // op_rb_rc_c3
    BR,

    // op_ra_rb_rc_c3
    BRL,

    // op_ra_rb_n
    SHR,
    SHRA,
    SHL,
    SHC,
}

// enum InstructionFormat {
//     OP,
//     OP_RA_RB_RC,
//     OP_RA_RB_C2,
//     OP_RA_C1,
//     OP_RA_RC,
//     OP_RB_RC_C3,
//     OP_RA_RB_RC_C3,
//     OP_RA_RB_N,
// }

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
    let (line, comment) = match line.find(';') {
        Some(x) => (&line[..x], Some(&line[x..])),
        None    => (&line[..], None),
    };
    let line = line.trim();

    let (line, label) = match line.find(':') {
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
        let (opcode, inst) = match inst.find(' ') {
            Some(x) => (&inst[..x], &inst[x..]),
            None => return Err("invalid")
        };
        let opcode = match Opcode::from_str(opcode) {
            Ok(x) => x,
            Err(_) => return Err("Opcode not found"),
        };
        let args = match opcode {
            Opcode::LA   => process_op_ra_rb_c2(inst, opcode),
            Opcode::NOP  => process_op(inst, opcode),
            _ => Err("temp"),
        };
        match inst {
            Ok(x) => Ok(Some(Instruction{
                opcode,
                args,
            })),
            Err(x) => Err(x)
        }
    }
}

fn process_args (inst: &str, opcode: Opcode) -> Result<Arguments, &'static str> {
    match opcode {
        Opcode::LA   => process_op_ra_rb_c2(inst, opcode),
        Opcode::NOP  => process_op(inst, opcode),
        _ => Err("temp"),
    };
}

    // OP,
    // OP_RA_RB_RC,
    // OP_RA_RB_C2,
    // OP_RA_C1,
    // OP_RA_RC,
    // OP_RB_RC_C3,
    // OP_RA_RB_RC_C3,
    // OP_RA_RB_N,

fn process_op (inst: &str, opcode: Opcode) -> Result<Arguments, &'static str> {
    match inst.is_empty() {
        true => Err("why are there arguments"),
        false => {
            Ok(Arguments {
                ra: None,
                rb: None,
                rc: None,
                c1: None,
                c2: None,
                c3: None,
            })
        },
    }
    
}

fn process_op_ra_rb_c2 (inst: &str, opcode: Opcode) -> Result<Arguments, &'static str> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[x..]),
        None => return Err("no comma"),
    };
    let rb = match (inst.find('('), inst.find('(')) {
        (Some(x), Some(y)) if (x < y) => &inst[x..y],
        _ => "",
    };

    let ra = match register_string_parse(ra) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    let rb = match register_string_parse(rb) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };


    Ok(Arguments {
        ra: ra,
        rb: rb,
        rc: None,
        c1: None,
        c2: None,
        c3: None,
    })
}

fn register_string_parse(reg: &str) -> Result<usize, &'static str> {
    if reg.len() < 2 || reg.len() > 3  {
        return Err("Incorrect register formatting");
    }

    let (a, b) = (&reg[0..0], &reg[1..]);

    let ret = b.parse::<usize>();

    match ret {
        Ok(x) => Ok(x),
        Err(x) => Err("Incorrect register formatting"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_op_test() {
        let result = process_op("nop", Opcode::NOP).unwrap();
        assert_eq!(result.ra, None);
        assert_eq!(result.rb, None);
        assert_eq!(result.rc, None);
        assert_eq!(result.c1, None);
        assert_eq!(result.c2, None);
        assert_eq!(result.c3, None);
    }

    #[test]
    fn register_string_parse_test() {
        for i in 0..32 {
            let input = format!("r{}", i);
            let result = register_string_parse(&input).unwrap();
            assert_eq!(i, result);
        }
    }
}     