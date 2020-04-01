use std::error::Error;
use std::str::FromStr;
use std::collections::HashMap;

pub struct InstLine<'a> {
    pub raw: &'a str,
    pub label: Option<&'a str>,
    inst: Option<Inst<'a>>,
    pub offset: Offset,
    comment: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub enum Offset {
    Relative(usize),
    Absolute(usize),
}

#[derive(Debug, PartialEq)]
struct Inst<'a> {
    opcode: Opcode,
    params: Params<'a>,
}

#[derive(Debug, PartialEq)]
struct Params<'a> {
    ra: Option<usize>,
    rb: Option<usize>,
    rc: Option<usize>,
    c1: Option<Con<'a>>,
    c2: Option<Con<'a>>,
    c3: Option<Con<'a>>,
}

#[derive(Debug, PartialEq)]
enum Con<'a> {
    C(usize),
    S(&'a str),
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

impl Opcode {
    fn to_num (self: &Self) -> usize {
        match self {
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
}

impl<'a> InstLine<'a> {
    pub fn new(inst: &'a str) -> Result<Option<InstLine<'a>>, Box<dyn Error>> {
        let raw = inst;
        let line = raw.trim();

        let (line, comment) = match line.find(';') {
            Some(x) => (line[..x].trim(), Some(line[(x+1)..].trim())),
            None    => (line[..].trim(), None),
        };

        let (line, label) = match line.find(':') {
            Some(x) => (line[(x+1)..].trim(), Some(line[..x].trim())),
            None    => (line[ ..].trim(), None)
        };

        let (inst, offset) = match line.find('.') {
            Some(x) => (None, process_directive(line)?),
            None => match process_instruction(line)? {
                Some(x) => (Some(x), Offset::Relative(4)),
                None => (None, Offset::Relative(0)),
            }, 
        };

        Ok(Some(
            InstLine{
                raw,
                label,
                inst,
                offset,
                comment,
            }
        ))
    }

    pub fn encode_instruction(self: &Self, symbol_map: &HashMap<&'a str, usize>, pc: usize) -> Result<Option<usize>, Box<dyn Error>> {
        match &self.inst.is_some() {
            true => match &self.inst {
                        Some(x) => Ok(Some(x.encode_instruction(symbol_map, pc)?)),
                        None => bail!("No instruction"),
                    },
            false => Ok(None),
        }
    }
}

impl<'a> Inst<'a> {
    pub fn encode_instruction(self: &Self, symbol_map: &HashMap<&'a str, usize>, pc: usize) -> Result<usize, Box<dyn Error>> {
        let op = self.opcode.to_num();
        let ra = match self.params.ra {
            Some(x) => x, 
            None => 0,
        };
        let rb = match self.params.rb {
            Some(x) => x, 
            None => 0,
        };
        let rc = match self.params.rc {
            Some(x) => x, 
            None => 0,
        };
        let c1 = match self.params.c1 {
            Some(Con::C(x)) => (x + (1<<22) - 4 - pc) % (1<<22),
            Some(Con::S(s)) => match symbol_map.get(s) {
                Some(x) => (*x + (1<<22) - 4 - pc) % (1<<22),
                None => bail!("Undefined symbol"),
            },
            None => 0,
        };
    
        let c2 = match self.params.c2 {
            Some(Con::C(x)) => x % (1<<17),
            Some(Con::S(s)) => match symbol_map.get(s) {
                Some(x) => *x,
                None => bail!("Undefined symbol"),
            },
            None => 0,
        };
        let c3 = match self.params.c3 {
            Some(Con::C(x)) => x % (1<<12),
            Some(Con::S(s)) => match symbol_map.get(s) {
                Some(x) => *x,
                None => bail!("Undefined symbol"),
            },
            None => 0,
        };

        let op = op << 27;
        let ra = ra << 22;
        let rb = rb << 17;
        let rc = rc << 12;

        print!("op:{}, r1:{}, r2:{}, r3:{}, c1:{}, c2:{}, c3:{}, sum:{}", op, ra, rb, rc, c1, c2, c3, op + ra + rb + rc + c1 + c2 + c3);

        Ok(op + ra + rb + rc + c1 + c2 + c3)
    }
}

fn process_instruction<'a>(inst: &'a str) -> Result<Option<Inst<'a>>, Box<dyn Error>> {
    if inst.is_empty() {
        Ok(None)
    } else {
        let (opcode, inst) = match inst.find(' ') {
            Some(x) => (&inst[ ..x], &inst[x.. ]),
            None =>    (&inst[ .. ], ""),
        };
        let opcode = match Opcode::from_str(&opcode.to_uppercase()) {
            Ok(x) => x,
            Err(_) => bail!("Opcode not found"),
        };
        let params = match process_params(inst, &opcode) {
            Ok(x) => x,
            Err(x) => return Err(x),
        };
        Ok(Some(Inst{
            opcode,
            params,
        }))
    }
}

fn process_directive<'a>(line: &str) -> Result<Offset, Box<dyn Error>> {
    let (dir, val) = match line.find(' ') {
        Some(x) => (line[..x].trim(), line[(x+1)..].trim()),
        None => bail!("Could not interpret directive (no parameter)"), 
    };
    match dir {
        ".org" => Ok(Offset::Absolute(val.parse::<usize>()?)),
        ".dw" => Ok(Offset::Relative(32 * val.parse::<usize>()?)),
        _ => bail!("Could not interpret directive (invalid directive)"),
    }
}

fn process_params<'a> (inst: &'a str, opcode: &Opcode) -> Result<Params<'a>, Box<dyn Error>> {
    match opcode {
        Opcode::NOP | Opcode::STOP 
            => process_op(inst),

        Opcode::ADD | Opcode::SUB | Opcode::AND | Opcode::OR 
            => process_op_ra_rb_rc(inst),

        Opcode::LD | Opcode::ST | Opcode::LA
            => process_op_ra_c2_rb(inst),

        Opcode::ADDI | Opcode::ANDI | Opcode::ORI
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

fn process_branch<'a> (inst: &'a str, opcode: &Opcode) -> Result<Params<'a>, Box<dyn Error>> {
    let (rb, rc) = match opcode {
        Opcode::BR => (inst, "r0"),
        Opcode::BRNV if inst.is_empty() => ("r0", "r0"),
        Opcode::BRZR | Opcode::BRNZ | Opcode::BRPL | Opcode::BRMI
            => match inst.find(',') {
                Some(x) => (&inst[..x], &inst[(x+1)..]),
                None => bail!("Could not parse temp"),
            },
        _ => bail!("Opcode could not be matched"),
    };

    let rb = match register_string_parse(rb) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };
    let rc = match register_string_parse(rc) {
        Ok(x) => Some(x),
        Err(x) => return Err(x),
    };

    let c3 = match opcode {
        Opcode::BRNV => Some(Con::C(0)),
        Opcode::BR   => Some(Con::C(1)),
        Opcode::BRZR => Some(Con::C(2)),
        Opcode::BRNZ => Some(Con::C(3)),
        Opcode::BRPL => Some(Con::C(4)),
        Opcode::BRMI => Some(Con::C(5)),
        _ => Some(Con::C(0)),
    };
    
    Ok(Params {
        ra: None,
        rb: rb,
        rc: rc,
        c1: None,
        c2: None,
        c3: c3,
    })
}

fn process_op (inst: &str) -> Result<Params, Box<dyn Error>> {
    let inst = inst.trim();
    match inst.is_empty() {
        false => bail!("why are there arguments"),
        true => {
            Ok(Params {
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

fn process_op_ra_rb_rc (inst: &str) -> Result<Params, Box<dyn Error>> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => bail!("no comma temp"),
    };
    let (rb, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => bail!("no comma temp"),
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

    Ok(Params {
        ra: ra,
        rb: rb,
        rc: rc,
        c1: None,
        c2: None,
        c3: None,
    })
}

fn process_op_ra_c2_rb (inst: &str) -> Result<Params, Box<dyn Error>> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => bail!("no comma temp"),
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

    Ok(Params {
        ra: ra,
        rb: rb,
        rc: None,
        c1: None,
        c2: c2,
        c3: None,
    })
}

fn process_op_ra_rb_c2 (inst: &str) -> Result<Params, Box<dyn Error>> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => bail!("no comma temp"),
    };
    let (rb, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => bail!("no comma temp"),
    };
    let c2 = inst;

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

    Ok(Params {
        ra: ra,
        rb: rb,
        rc: None,
        c1: None,
        c2: c2,
        c3: None,
    })
}

fn process_op_ra_c1 (inst: &str) -> Result<Params, Box<dyn Error>> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => bail!("no comma temp"),
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

    Ok(Params {
        ra: ra,
        rb: None,
        rc: None,
        c1: c1,
        c2: None,
        c3: None,
    })
}

fn process_op_ra_rc (inst: &str) -> Result<Params, Box<dyn Error>> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => bail!("no comma temp"),
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

    Ok(Params {
        ra: ra,
        rb: None,
        rc: rc,
        c1: None,
        c2: None,
        c3: None,
    })
}

fn process_op_ra_rb_rc_c3 (inst: &str) -> Result<Params, Box<dyn Error>> {
    let (ra, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => bail!("no comma temp"),
    };
    let (rb, inst) = match inst.find(',') {
        Some(x) => (&inst[..x], &inst[(x+1)..]),
        None => bail!("no comma temp"),
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
        (Ok(x), Err(_)) => (Some(x), Some(Con::C(0))),
        (Err(_), Ok(x)) => (Some(0), Some(x)),
        _ => bail!("Could not parse temp"),
    };

    Ok(Params {
        ra: ra,
        rb: rb,
        rc: rc,
        c1: None,
        c2: None,
        c3: c3,
    })
}

fn register_string_parse(reg: &str) -> Result<usize, Box<dyn Error>> {
    let reg = reg.trim();
    if reg.len() < 2 || reg.len() > 3  {
        println!("e:[{}]", reg);
        bail!("Incorrect register formatting (too many/few characters)");
    }

    let (a, b) = (reg.chars().next().unwrap(), &reg[1..]);

    if a != 'r' {
        bail!("Incorrect register formatting (does not start with 'r')");
    }

    let ret = b.parse::<usize>();

    match ret {
        Ok(x) if x < 32 => Ok(x),
        Ok(_) => bail!("Invalid register"),
        Err(_) => bail!("Incorrect register formatting (could not parse index)"),
    }
}

fn parse_constant<'a>(con: &'a str) -> Result<Con, Box<dyn Error>> {
    let con = con.trim();
    let ch = con.chars().next().unwrap();
    match ch {
        '-' => match con[1..].parse::<usize>() {
            Ok(x) => Ok(Con::C(!x + 1)),
            Err(_) => match con.chars().all(char::is_alphanumeric) {
                true => Ok(Con::S(con)),
                false => bail!(format!("Could not parse constant \"{}\"", con)),
            },
        },
        _ => match con.parse::<usize>() {
            Ok(x) => Ok(Con::C(x)),
            Err(_) => match con.chars().all(char::is_alphanumeric) {
                true => Ok(Con::S(con)),
                false => bail!(format!("Could not parse constant \"{}\"", con)),
            },
        }
    }
    
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn inst_line_new_test() {
        let tests = [
            ("add r1, r2, r3", None, Some(Inst{opcode: Opcode::ADD, params:Params{ra:Some(1), rb:Some(2), rc:Some(3), c1:None, c2:None, c3:None}}), None),
            ("LABEL: stop ; comment", Some("LABEL"), Some(Inst{opcode: Opcode::STOP, params:Params{ra:None, rb:None, rc:None, c1:None, c2:None, c3:None}}), Some("comment")),
        ];
        for test in &tests {
            let result = InstLine::new(test.0).unwrap().unwrap();

            assert_eq!(result.label, test.1);
            assert_eq!(result.inst, test.2);
            assert_eq!(result.comment, test.3);
        }
    }

    #[test]
    fn process_instruction_test() {
        let tests = [
            ("add r1, r2, r3", 0x60443000),
            ("andi r1,r1,1", 0xa8420001),
            ("brnz r31,r3", 0x403e3003),
            ("br r29", 0x403a0001),
            ("stop", 0xf8000000),
            ("st r1,0(r30)", 0x187c0000),
        ];
        for test in &tests {
            let result = process_instruction(test.0).unwrap().unwrap();
            let result = result.encode_instruction(&HashMap::new(), 0).unwrap();
            assert_eq!(result, test.1);
        }
    }

    #[test]
    fn process_directive_test() {
        let tests = [
            (".org 0", Offset::Absolute(0)),
            (".dw 10", Offset::Relative(320)),
        ];
        for test in &tests {
            let result = process_directive(test.0).unwrap();
            assert_eq!(result, test.1);
        }
    }

    #[test]
    fn process_branch_test() {
        let tests = [
            ("r1", &Opcode::BR, 
            Params{ra: None, rb: Some(1), rc: Some(0), c1: None, c2: None, c3: Some(Con::C(1))}),
            ("", &Opcode::BRNV, 
            Params{ra: None, rb: Some(0), rc: Some(0), c1: None, c2: None, c3: Some(Con::C(0))}),
            ("r1, r2", &Opcode::BRZR, 
            Params{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: Some(Con::C(2))}),
            ("r1, r2", &Opcode::BRNZ, 
            Params{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: Some(Con::C(3))}),
            ("r1, r2", &Opcode::BRPL, 
            Params{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: Some(Con::C(4))}),
            ("r1, r2", &Opcode::BRMI,
            Params{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: Some(Con::C(5))}),
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
    fn process_op_test() {
        let tests = [
            ("", 
            Params{ra: None, rb: None, rc: None, c1: None, c2: None, c3: None}),
            (" ", 
            Params{ra: None, rb: None, rc: None, c1: None, c2: None, c3: None}),
        ];
        for test in &tests {
            let result = process_op(test.0).unwrap();
            assert_eq!(result, test.1);
        }

        let invalid_tests = [
            "test"
        ];
        for test in &invalid_tests {
            let result = process_op(test);
            assert!(result.is_err());
        }
    }

    #[test]
    fn process_op_ra_rb_rc_test() {
        let tests = [
            ("r1, r2, r3",
            Params{ra: Some(1), rb: Some(2), rc: Some(3), c1: None, c2: None, c3: None}),
            ("  r23,   r6,r11 ", 
            Params{ra: Some(23), rb: Some(6), rc: Some(11), c1: None, c2: None, c3: None}),
        ];
        for test in &tests {
            let result = process_op_ra_rb_rc(test.0).unwrap();
            assert_eq!(result, test.1);
        }

        let invalid_tests = [
            "test",
            "r1, r3, r3123",
            "r2,,r3 r3",
            "a3,b3",
        ];
        for test in &invalid_tests {
            let result = process_op_ra_rb_rc(test);
            assert!(result.is_err(), format!("failed with [{}]", test));
        }
    }

    #[test]
    fn process_op_ra_c2_rb_test() {
        let tests = [
            ("r1, 5(r2)",
            Params{ra: Some(1), rb: Some(2), rc: None, c1: None, c2: Some(Con::C(5)), c3: None}),
            ("  r23,   16 ( r11 ) ",
            Params{ra: Some(23), rb: Some(11), rc: None, c1: None, c2: Some(Con::C(16)), c3: None}),
            ("  r23 ,16( r11)",
            Params{ra: Some(23), rb: Some(11), rc: None, c1: None, c2: Some(Con::C(16)), c3: None}),
            ("r1, 5",
            Params{ra: Some(1), rb: Some(0), rc: None, c1: None, c2: Some(Con::C(5)), c3: None}),
        ];
        for test in &tests {
            let result = process_op_ra_c2_rb(test.0).unwrap();
            assert_eq!(result, test.1);
        }

        let invalid_tests = [
            "test",
            "r1, r3, r3123",
            "r2,,r3 r3",
            "a3,b3",
        ];
        for test in &invalid_tests {
            let result = process_op_ra_c2_rb(test);
            assert!(result.is_err(), format!("failed with [{}]", test));
        }
    }

    #[test]
    fn process_op_ra_rb_c2_test() {
        let tests = [
            ("r1, r2, 5",
            Params{ra: Some(1), rb: Some(2), rc: None, c1: None, c2: Some(Con::C(5)), c3: None}),
            ("  r23,     r11 , 16 ",
            Params{ra: Some(23), rb: Some(11), rc: None, c1: None, c2: Some(Con::C(16)), c3: None}),
            ("  r23 , r11 ,16",
            Params{ra: Some(23), rb: Some(11), rc: None, c1: None, c2: Some(Con::C(16)), c3: None}),
            ("r1, r1, 16",
            Params{ra: Some(1), rb: Some(1), rc: None, c1: None, c2: Some(Con::C(16)), c3: None}),
        ];
        for test in &tests {
            let result = process_op_ra_rb_c2(test.0).unwrap();
            assert_eq!(result, test.1);
        }

        let invalid_tests = [
            "test",
            "r1, r3,,389",
            "r2,,r3 r3",
            "a3,b3",
        ];
        for test in &invalid_tests {
            let result = process_op_ra_rb_c2(test);
            assert!(result.is_err(), format!("failed with [{}]", test));
        }
    }

    #[test]
    fn process_op_ra_c1_test() {
        let tests = [
            ("r1, 5",
            Params{ra: Some(1),  rb: None, rc: None, c1: Some(Con::C(5)),  c2: None, c3: None}),
            ("  r23,   16",
            Params{ra: Some(23), rb: None, rc: None, c1: Some(Con::C(16)), c2: None, c3: None}),
            ("  r23,16",
            Params{ra: Some(23), rb: None, rc: None, c1: Some(Con::C(16)), c2: None, c3: None}),
        ];
        for test in &tests {
            let result = process_op_ra_c1(test.0).unwrap();
            assert_eq!(result, test.1);
        }

        let invalid_tests = [
            "test",
            "r1, 31a23",
            "r2,3,,,r3",
            "a3,5",
        ];
        for test in &invalid_tests {
            let result = process_op_ra_c1(test);
            assert!(result.is_err(), format!("failed with [{}]", test));
        }
    }

    #[test]
    fn process_op_ra_rc_test() {
        let tests = [
            ("r1, r3",
            Params{ra: Some(1), rb: None, rc: Some(3), c1: None, c2: None, c3: None}),
            ("  r23,   r6",
            Params{ra: Some(23), rb: None, rc: Some(6), c1: None, c2: None, c3: None}),
            ("  r23,r31",
            Params{ra: Some(23), rb: None, rc: Some(31), c1: None, c2: None, c3: None}),
        ];
        for test in &tests {
            let result = process_op_ra_rc(test.0).unwrap();
            assert_eq!(result, test.1);
        }

        let invalid_tests = [
            "test",
            "r1, r31a23",
            "r2,r3,,,r3",
            "a3,r5",
        ];
        for test in &invalid_tests {
            let result = process_op_ra_rc(test);
            assert!(result.is_err(), format!("failed with [{}]", test));
        }
    }

    #[test]
    fn process_op_ra_rb_rc_c3_test() {
        let tests = [
            ("r1, r2, r3",
            Params{ra: Some(1), rb: Some(2), rc: Some(3), c1: None, c2: None, c3: Some(Con::C(0))}),
            ("r1, r2, 3",
            Params{ra: Some(1), rb: Some(2), rc: Some(0), c1: None, c2: None, c3: Some(Con::C(3))}),
            (" r1,r2,  r3 ",
            Params{ra: Some(1), rb: Some(2), rc: Some(3), c1: None, c2: None, c3: Some(Con::C(0))}),
            ("r1,r2,3 ",
            Params{ra: Some(1), rb: Some(2), rc: Some(0), c1: None, c2: None, c3: Some(Con::C(3))}),
        ];
        for test in &tests {
            let result = process_op_ra_rb_rc_c3(test.0).unwrap();
            assert_eq!(result, test.1);
        }

        let invalid_tests = [
            "test",
            "r1, r31,a23",
            "r2,r3,,,r3",
            "a3,r5,5",
        ];
        for test in &invalid_tests {
            let result = process_op_ra_rb_rc_c3(test);
            assert!(result.is_err(), format!("failed with [{}]", test));
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
