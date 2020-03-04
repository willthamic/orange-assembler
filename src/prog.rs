use std::error::Error;
use crate::inst;
use std::collections::HashMap;

pub struct Prog<'a> {
    pub name: &'a str,
    lines: Vec<(usize, inst::InstLine<'a>)>,
    symbol_map: HashMap<&'a str, usize>,
}

impl<'a> Prog<'a> {
    pub fn new (name: &'a str, contents: &'a str) -> Result<Prog<'a>, Box<dyn Error>> {
        let source_lines = contents.lines();
        let mut lines = Vec::new();

        let mut symbol_map = HashMap::new();
        let mut loc_counter = 0;

        for line in source_lines {
            match inst::InstLine::new(&line)? {
                Some(inst_line) => {
                    let loc_counter_temp = match inst_line.offset {
                        inst::Offset::Relative(x) => loc_counter + x,
                        inst::Offset::Absolute(x) => x,
                    };
                    match inst_line.label {
                        Some(label) => match symbol_map.insert(label, loc_counter) {
                            Some(_) => bail!("Encountered duplicate symbol"),
                            None => (),
                        },
                        None => (),
                    }
                    lines.push((loc_counter, inst_line));
                    loc_counter = loc_counter_temp;
                },
                None => (),
            }
        }
        println!("\n{:?}", symbol_map);

        Ok(Prog {
            name,
            lines,
            symbol_map,
        })
    }

    
    pub fn encode (self: &Self) -> Result<String, Box<dyn Error>> {
        let mut s: String = String::from("00000000\n");
        for line in &self.lines {
            match line.1.encode_instruction(&self.symbol_map, line.0)? {
                Some(x) => s.push_str(&format!("{:08x}\t{:08x}\n", line.0, x)),
                None => (),
            }
            println!("");
        }

        Ok(s)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn prog_new_test() {
        let tests = [
            ("add r1,r2,r3", "00000000\n00000000\t60443000\n"),
            ("add r1,r2,r3\n add r1,r2,r3", "00000000\n00000000\t60443000\n00000004\t60443000\n"),
            ("LABEL: addi r1,r2,LABEL", "00000000\n00000000\t68440000\n"),
        ];
        for test in &tests {
            let result = Prog::new("file", test.0).unwrap().encode().unwrap();

            assert_eq!(result, test.1);
        }
    }

}