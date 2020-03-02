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

// impl fmt::Debug for Instruction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f, "{}{:?}{:?}{:?}{:?}{:?}{:?}", 
//             self.opcode, 
//             self.args.ra, 
//             self.args.rb, 
//             self.args.rc, 
//             self.args.c1, 
//             self.args.c2, 
//             self.args.c3
//         )
//     }
// }

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
            => process_op(inst, opcode),

        Opcode::ADD | Opcode::SUB | Opcode::AND | Opcode::OR 
            => process_op_ra_rb_rc(inst, &opcode),

        Opcode::ADDI | Opcode::ANDI | Opcode::ORI | Opcode::LD | Opcode::ST | Opcode::LA
            => process_op_ra_rb_c2(inst, &opcode),

        Opcode::LDR | Opcode::STR | Opcode::LAR
            => process_op_ra_c1(inst, &opcode),
        
        Opcode::NEG | Opcode::NOT
            => process_op_ra_rc(inst, &opcode),

        Opcode::BR | Opcode::BRNV | Opcode::BRZR | Opcode::BRNZ | Opcode::BRPL | Opcode::BRMI
            => process_branch(inst, &opcode),

        Opcode::SHR | Opcode::SHRA | Opcode::SHL | Opcode::SHC
            => process_op_ra_rb_rc_c3(inst, &opcode),
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

    // OP,
    // OP_RA_RB_RC,
    // OP_RA_RB_C2,
    // OP_RA_C1,
    // OP_RA_RC,
    // OP_RB_RC_C3,
    // OP_RA_RB_RC_C3,
    // OP_RA_RB_C3,

fn process_op (inst: &str, opcode: &Opcode) -> Result<Arguments, &'static str> {
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

fn process_op_ra_rb_rc (inst: &str, opcode: &Opcode) -> Result<Arguments, &'static str> {
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

fn process_op_ra_rb_c2 (inst: &str, opcode: &Opcode) -> Result<Arguments, &'static str> {
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

fn process_op_ra_c1 (inst: &str, opcode: &Opcode) -> Result<Arguments, &'static str> {
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

fn process_op_ra_rc (inst: &str, opcode: &Opcode) -> Result<Arguments, &'static str> {
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

fn process_op_ra_rb_rc_c3 (inst: &str, opcode: &Opcode) -> Result<Arguments, &'static str> {
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
        Ok(x) => Err("Invalid register"),
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
mod tests {
    use super::*;

    #[test]
    fn process_op_test() {
        let tests = [
            ("", &Opcode::NOP, 
            Arguments{ra: None, rb: None, rc: None, c1: None, c2: None, c3: None}),
            ("", &Opcode::STOP, 
            Arguments{ra: None, rb: None, rc: None, c1: None, c2: None, c3: None}),
            (" ", &Opcode::NOP, 
            Arguments{ra: None, rb: None, rc: None, c1: None, c2: None, c3: None}),
        ];
        for test in &tests {
            let result = process_op(test.0, test.1).unwrap();
            assert_eq!(result, test.2);
        }

        let invalid_tests = [
            ("test", &Opcode::NOP)
        ];
        for test in &invalid_tests {
            let result = process_op(test.0, test.1);
            assert!(result.is_err());
        }
    }

    #[test]
    fn process_op_ra_rb_rc_test() {
        let tests = [
            ("r1, r2, r3", &Opcode::ADD, 
            Arguments{ra: Some(1), rb: Some(2), rc: Some(3), c1: None, c2: None, c3: None}),
            ("  r23,   r6,r11 ", &Opcode::SUB, 
            Arguments{ra: Some(23), rb: Some(6), rc: Some(11), c1: None, c2: None, c3: None}),
        ];
        for test in &tests {
            let result = process_op_ra_rb_rc(test.0, test.1).unwrap();
            assert_eq!(result, test.2);
        }

        let invalid_tests = [
            ("test", &Opcode::NOP),
            ("r1, r3, r3123", &Opcode::NOP),
            ("r2,,r3 r3", &Opcode::NOP),
            ("a3,b3", &Opcode::NOP),
        ];
        for test in &invalid_tests {
            let result = process_op_ra_rb_rc(test.0, test.1);
            assert!(result.is_err(), format!("failed with [{}]", test.0));
        }
    }

    #[test]
    fn process_op_ra_rb_c2_test() {
        let tests = [
            ("r1, 5(r2)", &Opcode::LD, 
            Arguments{ra: Some(1), rb: Some(2), rc: None, c1: None, c2: Some(5), c3: None}),
            ("  r23,   16 ( r11 ) ", &Opcode::ST, 
            Arguments{ra: Some(23), rb: Some(11), rc: None, c1: None, c2: Some(16), c3: None}),
            ("  r23 ,16( r11)", &Opcode::ST, 
            Arguments{ra: Some(23), rb: Some(11), rc: None, c1: None, c2: Some(16), c3: None}),
            ("r1, 5", &Opcode::ST, 
            Arguments{ra: Some(1), rb: Some(0), rc: None, c1: None, c2: Some(5), c3: None}),
        ];
        for test in &tests {
            let result = process_op_ra_rb_c2(test.0, test.1).unwrap();
            assert_eq!(result, test.2);
        }

        let invalid_tests = [
            ("test", &Opcode::NOP),
            ("r1, r3, r3123", &Opcode::NOP),
            ("r2,,r3 r3", &Opcode::NOP),
            ("a3,b3", &Opcode::NOP),
        ];
        for test in &invalid_tests {
            let result = process_op_ra_rb_c2(test.0, test.1);
            assert!(result.is_err(), format!("failed with [{}]", test.0));
        }
    }

    #[test]
    fn process_op_ra_c1_test() {
        let tests = [
            ("r1, 5", &Opcode::LDR, 
            Arguments{ra: Some(1), rb: None, rc: None, c1: Some(5), c2: None, c3: None}),
            ("  r23,   16", &Opcode::STR, 
            Arguments{ra: Some(23), rb: None, rc: None, c1: Some(16), c2: None, c3: None}),
            ("  r23,16", &Opcode::LDR, 
            Arguments{ra: Some(23), rb: None, rc: None, c1: Some(16), c2: None, c3: None}),
        ];
        for test in &tests {
            let result = process_op_ra_c1(test.0, test.1).unwrap();
            assert_eq!(result, test.2);
        }

        let invalid_tests = [
            ("test", &Opcode::NOP),
            ("r1, 31a23", &Opcode::NOP),
            ("r2,3,,,r3", &Opcode::NOP),
            ("a3,5", &Opcode::NOP),
        ];
        for test in &invalid_tests {
            let result = process_op_ra_c1(test.0, test.1);
            assert!(result.is_err(), format!("failed with [{}]", test.0));
        }
    }

    #[test]
    fn process_op_ra_rc_test() {
        let tests = [
            ("r1, r3", &Opcode::LDR, 
            Arguments{ra: Some(1), rb: None, rc: Some(3), c1: None, c2: None, c3: None}),
            ("  r23,   r6", &Opcode::STR, 
            Arguments{ra: Some(23), rb: None, rc: Some(6), c1: None, c2: None, c3: None}),
            ("  r23,r31", &Opcode::LDR, 
            Arguments{ra: Some(23), rb: None, rc: Some(31), c1: None, c2: None, c3: None}),
        ];
        for test in &tests {
            let result = process_op_ra_rc(test.0, test.1).unwrap();
            assert_eq!(result, test.2);
        }

        let invalid_tests = [
            ("test", &Opcode::NOP),
            ("r1, r31a23", &Opcode::NOP),
            ("r2,r3,,,r3", &Opcode::NOP),
            ("a3,r5", &Opcode::NOP),
        ];
        for test in &invalid_tests {
            let result = process_op_ra_rc(test.0, test.1);
            assert!(result.is_err(), format!("failed with [{}]", test.0));
        }
    }

    #[test]
    fn process_op_ra_rb_rc_c3_test() {
        let tests = [
            ("r1, r2, r3", &Opcode::SHR, 
            Arguments{ra: Some(1), rb: Some(2), rc: Some(3), c1: None, c2: None, c3: Some(0)}),
            ("r1, r2, 3", &Opcode::SHR, 
            Arguments{ra: Some(1), rb: Some(2), rc: Some(0), c1: None, c2: None, c3: Some(3)}),
            (" r1,r2,  r3 ", &Opcode::SHR, 
            Arguments{ra: Some(1), rb: Some(2), rc: Some(3), c1: None, c2: None, c3: Some(0)}),
            ("r1,r2,3 ", &Opcode::SHR, 
            Arguments{ra: Some(1), rb: Some(2), rc: Some(0), c1: None, c2: None, c3: Some(3)}),
        ];
        for test in &tests {
            let result = process_op_ra_rb_rc_c3(test.0, test.1).unwrap();
            assert_eq!(result, test.2);
        }

        let invalid_tests = [
            ("test", &Opcode::NOP),
            ("r1, r31,a23", &Opcode::NOP),
            ("r2,r3,,,r3", &Opcode::NOP),
            ("a3,r5,5", &Opcode::NOP),
        ];
        for test in &invalid_tests {
            let result = process_op_ra_rb_rc_c3(test.0, test.1);
            assert!(result.is_err(), format!("failed with [{}]", test.0));
        }
    }

    #[test]
    fn process_branch_test() {
        let tests = [
            ("r1", &Opcode::BR, 
            Arguments{ra: None, rb: Some(1), rc: Some(0), c1: None, c2: None, c3: None}),
            ("", &Opcode::BRNV, 
            Arguments{ra: None, rb: Some(0), rc: Some(0), c1: None, c2: None, c3: None}),
            ("r1, r2", &Opcode::BRZR, 
            Arguments{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: None}),
            ("r1, r2", &Opcode::BRNZ, 
            Arguments{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: None}),
            ("r1, r2", &Opcode::BRPL, 
            Arguments{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: None}),
            ("r1, r2", &Opcode::BRMI, 
            Arguments{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: None}),
        ];
        for test in &tests {
            let result = process_branch(test.0, test.1).unwrap();
            assert_eq!(result, test.2);
        }

        let invalid_tests = [
            ("test", &Opcode::NOP),
            ("r1, r31", &Opcode::BR),
            ("r1", &Opcode::BRNV),
            ("a3,r5", &Opcode::BRNZ),
            ("r3,, r5", &Opcode::BRNZ),
            ("r3,r5,r6", &Opcode::BRNZ),
        ];
        for test in &invalid_tests {
            let result = process_branch(test.0, test.1);
            assert!(result.is_err(), format!("failed with [{}]", test.0));
        }
    }

    #[test]
    fn register_string_parse_test() {
        for i in 0..32 {
            let input = format!("r{}", i);
            let result = register_string_parse(&input).unwrap();
            assert_eq!(i, result);
        }

        let invalid_tests = [
            "a23",
            "123",
            "",
            "rrr",
            "r 23",
            "r32",
            "r-2",
        ];
        for test in &invalid_tests {
            let result = register_string_parse(test);
            assert!(result.is_err(), format!("failed with [{}]", test));
        }
    }
}     