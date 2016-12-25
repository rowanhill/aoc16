use parser::Operand::*;
use parser::Instruction::*;

use regex::Regex;

lazy_static! {
    static ref CPY_RE:Regex = Regex::new(r"cpy (.+?) (.+)").unwrap();
    static ref INC_RE:Regex = Regex::new(r"inc (.+?)").unwrap();
    static ref DEC_RE:Regex = Regex::new(r"dec (.+?)").unwrap();
    static ref JNZ_RE:Regex = Regex::new(r"jnz (.+?) (.+)").unwrap();
    static ref TGL_RE:Regex = Regex::new(r"tgl (.+?)").unwrap();
    static ref OUT_RE:Regex = Regex::new(r"out (.+?)").unwrap();
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Register(usize),
    Literal(i32)
}

impl Operand {
    pub fn parse(str: &str) -> Operand {
        match str {
            "a" => Register(0),
            "b" => Register(1),
            "c" => Register(2),
            "d" => Register(3),
            _   => {
                if let Ok(val) = str.parse::<i32>() {
                    Literal(val)
                } else {
                    unreachable!("Unexpected operand: {}", str);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Copy{source:Operand, target:Operand},
    Inc{reg:Operand},
    Dec{reg:Operand},
    JumpNotZero{check:Operand, delta:Operand},
    Toggle{reg:Operand},
    Out{operand:Operand},
    MultiplyAddAndClear{factor_1:Operand, factor_2:Operand, target:Operand, clear:Operand},
    AddAndClear{source:Operand, target:Operand, clear:Operand},
    Nop
}

impl Instruction {
    pub fn parse(line: &str) -> Instruction {
        if let Some(caps) = CPY_RE.captures(line) {
            let val_or_reg = Operand::parse(caps.at(1).unwrap());
            let target_reg = Operand::parse(caps.at(2).unwrap());
            Copy{source: val_or_reg, target: target_reg}
        } else if let Some(caps) = INC_RE.captures(line) {
            let reg = Operand::parse(caps.at(1).unwrap());
            Inc{reg: reg}
        } else if let Some(caps) = DEC_RE.captures(line) {
            let reg = Operand::parse(caps.at(1).unwrap());
            Dec{reg: reg}
        } else if let Some(caps) = JNZ_RE.captures(line) {
            let val_or_reg = Operand::parse(caps.at(1).unwrap());
            let delta = Operand::parse(caps.at(2).unwrap());
            JumpNotZero{check: val_or_reg, delta: delta}
        } else if let Some(caps) = TGL_RE.captures(line) {
            let reg = Operand::parse(caps.at(1).unwrap());
            Toggle{reg: reg}
        } else if let Some(caps) = OUT_RE.captures(line) {
            let operand = Operand::parse(caps.at(1).unwrap());
            Out{operand: operand}
        } else {
            unreachable!("Did not recognise instruction: {}", line);
        }
    }

    pub fn toggle(&self) -> Instruction {
        match *self {
            Copy{source, target} => {
                JumpNotZero{check: source, delta: target}
            },
            Inc{reg} => {
                Dec{reg: reg}
            },
            Dec{reg} => {
                Inc{reg: reg}
            },
            JumpNotZero{check, delta} => {
                Copy{source: check, target: delta}
            },
            Toggle{reg} => {
                Inc { reg: reg }
            },
            Out{operand} => {
                Inc { reg: operand }
            },
            MultiplyAddAndClear{..} | AddAndClear{..} | Nop => {
                unreachable!("Trying to toggle an optimised instruction")
            }
        }
    }
}