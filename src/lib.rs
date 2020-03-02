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

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: Opcode,
    args: Arguments,
}

#[derive(Debug, PartialEq)]
struct Arguments {
    ra: Option<usize>,
    rb: Option<usize>,
    rc: Option<usize>,
    c1: Option<usize>,
    c2: Option<usize>,
    c3: Option<usize>,
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

    // op_ra_rb_rc_c3
    BR,
    BRNV,
    BRZR,
    BRNZ,
    BRPL,
    BRMI,

    // op_ra_rb_n
    SHR,
    SHRA,
    SHL,
    SHC,
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

    let ilines = process_program(&contents)?;

    let encoded = encode_ilines(&ilines);

    fs::write(config.output_path, encoded)
        .expect("Unable to write file");

    Ok(())
}

fn process_program (contents: &str) -> Result<Vec::<ILine>, &'static str> {
    let lines = contents.lines();
    let mut ilines = Vec::<ILine>::new();

    for line in lines {
        match process_line(line) {
            Ok(Some(x)) => ilines.push(x),
            Ok(None) => (),
            Err(x) => return Err(x),
        }
    }

    Ok(ilines)
}

fn encode_ilines (ilines: &Vec::<ILine>) -> String {
    let mut s: String = String::new();
    for iline in ilines {
        match &iline.instruction {
            Some(x) => s.push_str(&encode_instruction(x)),
            None => (),
        }
    }

    s
}

fn encode_instruction (inst: &Instruction) -> String {
    let op = opcode_to_num(&inst.opcode);
    let ra = match inst.args.ra {
        Some(x) => x, 
        None => 0,
    };
    let rb = match inst.args.rb {
        Some(x) => x, 
        None => 0,
    };
    let rc = match inst.args.rc {
        Some(x) => x, 
        None => 0,
    };
    let c1 = match inst.args.c1 {
        Some(x) => x, 
        None => 0,
    };
    let c2 = match inst.args.c2 {
        Some(x) => x, 
        None => 0,
    };
    let c3 = match inst.args.c3 {
        Some(x) => x, 
        None => 0,
    };

    let op = op << 27;
    let ra = ra << 22;
    let rb = rb << 17;
    let rc = rc << 12;

    let ret = op + ra + rb + rc + c1 + c2 + c3;

    format!("{:X}", ret)
}

fn opcode_to_num (opcode: &Opcode) -> usize {
    match opcode {
        Opcode::NOP  => 0, 
        Opcode::LD   => 1,
        Opcode::LDR  => 2,
        Opcode::ST   => 3,
        Opcode::STR  => 4,
        Opcode::LA   => 5,
        Opcode::LAR  => 6,
        Opcode::BR   => 8,
        Opcode::BRNV => 8,
        Opcode::BRZR => 8,
        Opcode::BRNZ => 8,
        Opcode::BRPL => 8,
        Opcode::BRMI => 8,
        Opcode::ADD  => 12, 
        Opcode::ADDI => 13,
        Opcode::SUB  => 14, 
        Opcode::NEG  => 15,
        Opcode::AND  => 20, 
        Opcode::ANDI => 21,
        Opcode::OR   => 22,
        Opcode::ORI  => 23,
        Opcode::NOT  => 24,
        Opcode::SHR  => 26,
        Opcode::SHRA => 27,
        Opcode::SHL  => 28,
        Opcode::SHC  => 29,
        Opcode::STOP => 31,
    }
}

fn process_line (raw: &str) -> Result<Option<ILine>, &'static str> {
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
        let opcode = match Opcode::from_str(&opcode.to_uppercase()) {
            Ok(x) => x,
            Err(_) => return Err("Opcode not found"),
        };
        let args = match process_args(inst, &opcode) {
            Ok(x) => x,
            Err(x) => return Err(x),
        };
        Ok(Some(Instruction{
            opcode,
            args,
        }))
    }
}

fn process_args (inst: &str, opcode: &Opcode) -> Result<Arguments, &'static str> {
    match opcode {
        Opcode::NOP | Opcode::STOP 
            => process_op(inst),

        Opcode::ADD | Opcode::SUB | Opcode::AND | Opcode::OR 
            => process_op_ra_rb_rc(inst),

        Opcode::ADDI | Opcode::ANDI | Opcode::ORI | Opcode::LD | Opcode::ST | Opcode::LA
            => process_op_ra_rb_c2(inst),

        Opcode::LDR | Opcode::STR | Opcode::LAR
            => process_op_ra_c1(inst),
        
        Opcode::NEG | Opcode::NOT
            => process_op_ra_rc(inst),

        Opcode::BR | Opcode::BRNV | Opcode::BRZR | Opcode::BRNZ | Opcode::BRPL | Opcode::BRMI
            => process_branch(inst, &opcode),

        Opcode::SHR | Opcode::SHRA | Opcode::SHL | Opcode::SHC
            => process_op_ra_rb_rc_c3(inst),
    }
}

fn process_branch (inst: &str, opcode: &Opcode) -> Result<Arguments, &'static str> {
    let (rb, rc) = match opcode {
        Opcode::BR => (inst, "r0"),
        Opcode::BRNV if inst.is_empty() => ("r0", "r0"),
        Opcode::BRZR | Opcode::BRNZ | Opcode::BRPL | Opcode::BRMI
            => match inst.find(',') {
                Some(x) => (&inst[..x], &inst[(x+1)..]),
                None => return Err("Could not parse temp"),
            },
        _ => return Err("Opcode could not be matched"),
    };

    let rb = match register_string_parse(rb) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    let rc = match register_string_parse(rc) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    
    Ok(Arguments {
        ra: None,
        rb: rb,
        rc: rc,
        c1: None,
        c2: None,
        c3: None,
    })
}

fn process_op (inst: &str) -> Result<Arguments, &'static str> {
    let inst = inst.trim();
    match inst.is_empty() {
        false => Err("why are there arguments"),
        true => {
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

fn process_op_ra_rb_rc (inst: &str) -> Result<Arguments, &'static str> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => return Err("no comma temp"),
    };
    let (rb, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => return Err("no comma temp"),
    };
    let rc = inst;

    let ra = match register_string_parse(ra) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    let rb = match register_string_parse(rb) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    let rc = match register_string_parse(rc) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };

    Ok(Arguments {
        ra: ra,
        rb: rb,
        rc: rc,
        c1: None,
        c2: None,
        c3: None,
    })
}

fn process_op_ra_rb_c2 (inst: &str) -> Result<Arguments, &'static str> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => return Err("no comma temp"),
    };
    let (c2, rb) = match (inst.find('('), inst.find(')')) {
        (Some(x), Some(y)) if (x < y) => (&inst[..x], &inst[(x+1)..y]),
        _ => (inst, "r0"),
    };

    let ra = match register_string_parse(ra) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    let rb = match register_string_parse(rb) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    let c2 = match parse_constant(c2) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };

    Ok(Arguments {
        ra: ra,
        rb: rb,
        rc: None,
        c1: None,
        c2: c2,
        c3: None,
    })
}

fn process_op_ra_c1 (inst: &str) -> Result<Arguments, &'static str> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => return Err("no comma temp"),
    };
    let c1 = inst;

    let ra = match register_string_parse(ra) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    let c1 = match parse_constant(c1) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };

    Ok(Arguments {
        ra: ra,
        rb: None,
        rc: None,
        c1: c1,
        c2: None,
        c3: None,
    })
}

fn process_op_ra_rc (inst: &str) -> Result<Arguments, &'static str> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => return Err("no comma temp"),
    };
    let rc = inst;

    let ra = match register_string_parse(ra) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    let rc = match register_string_parse(rc) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };

    Ok(Arguments {
        ra: ra,
        rb: None,
        rc: rc,
        c1: None,
        c2: None,
        c3: None,
    })
}

fn process_op_ra_rb_rc_c3 (inst: &str) -> Result<Arguments, &'static str> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => return Err("no comma temp"),
    };
    let (rb, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => return Err("no comma temp"),
    };

    let ra = match register_string_parse(ra) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    let rb = match register_string_parse(rb) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    let (rc, c3) = match (register_string_parse(inst), parse_constant(inst)) {
        (Ok(x), Err(_)) => (Some(x), Some(0)),
        (Err(_), Ok(x)) => (Some(0), Some(x)),
        _ => return Err("Could not parse temp"),
    };

    Ok(Arguments {
        ra: ra,
        rb: rb,
        rc: rc,
        c1: None,
        c2: None,
        c3: c3,
    })
}

fn register_string_parse(reg: &str) -> Result<usize, &'static str> {
    let reg = reg.trim();
    if reg.len() < 2 || reg.len() > 3  {
        println!("e:[{}]", reg);
        return Err("Incorrect register formatting (too many/few characters)");
    }

    let (a, b) = (reg.chars().next().unwrap(), &reg[1..]);

    if a != 'r' {
        return Err("Incorrect register formatting (does not start with 'r')");
    }

    let ret = b.parse::<usize>();

    match ret {
        Ok(x) if x < 32 => Ok(x),
        Ok(_) => Err("Invalid register"),
        Err(_) => Err("Incorrect register formatting (could not parse index)"),
    }
}

fn parse_constant(con: &str) -> Result<usize, &'static str> {
    let con = con.trim();
    let con = con.parse::<usize>();
    match con {
        Ok(x) => Ok(x),
        Err(_) => Err("Could not parse constant"),
    }
}

#[cfg(test)]
mod test;
