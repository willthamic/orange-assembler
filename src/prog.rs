use crate::inst;
use std::collections::HashMap;

pub struct Prog<'a> {
    pub name: &'a str,
    inst_lines: Vec<inst::InstLine<'a>>,
    symbol_map: HashMap<&'a str, usize>,
}

impl<'a> Prog<'a> {
    pub fn new (name: &'a str, contents: &'a str) -> Result<Prog<'a>, &'static str> {
        let lines = contents.lines();
        let mut inst_lines = Vec::<inst::InstLine>::new();

        for line in lines {
            match inst::InstLine::new(line) {
                Ok(Some(x)) => inst_lines.push(x),
                Ok(None) => (),
                Err(x) => return Err(x),
            }
        }

        Ok(Prog {
            name,
            inst_lines,
        })
    }

    
    pub fn encode (self: &Self) -> Result<String,&'static str> {
        let mut s: String = String::new();
        for iline in &self.inst_lines {
            if iline.has_inst() {
                s.push_str(&iline.encode_instruction()?);
            }
        }

        Ok(s)
    }
}

