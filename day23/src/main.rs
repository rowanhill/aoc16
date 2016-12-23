extern crate regex;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

use Operand::*;
use Instruction::*;

lazy_static! {
    static ref CPY_RE:Regex = Regex::new(r"cpy (.+?) (.+)").unwrap();
    static ref INC_RE:Regex = Regex::new(r"inc (.+?)").unwrap();
    static ref DEC_RE:Regex = Regex::new(r"dec (.+?)").unwrap();
    static ref JNZ_RE:Regex = Regex::new(r"jnz (.+?) (.+)").unwrap();
    static ref TGL_RE:Regex = Regex::new(r"tgl (.+?)").unwrap();
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(usize),
    Literal(i32)
}

impl Operand {
    fn parse(str: &str) -> Operand {
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
enum Instruction {
    Copy{source:Operand, target:Operand},
    Inc{reg:Operand},
    Dec{reg:Operand},
    JumpNotZero{check:Operand, delta:Operand},
    Toggle{reg:Operand}
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
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
        } else {
            unreachable!("Did not recognise instruction: {}", line);
        }
    }

    fn toggle(&self) -> Instruction {
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
            }
        }
    }
}

fn parse(file: &str) -> Vec<Instruction> {
    file.lines().map(|line| Instruction::parse(line)).collect()
}

struct Cpu {
    regs: [i32; 4]
}

impl Cpu {
    fn new(a_val: i32) -> Cpu {
        Cpu {
            regs: [a_val, 0, 0, 0]
        }
    }

    fn value(&self, reg_or_val: Operand) -> i32 {
        match reg_or_val {
            Register(reg_idx) => self.regs[reg_idx],
            Literal(value) => value
        }
    }

    fn set_value(&mut self, reg: Operand, reg_or_val: Operand) {
        if let Register(reg_idx) = reg {
            let value = self.value(reg_or_val);
            self.regs[reg_idx] = value;
        } else {
            panic!("Trying to set a value to a literal: {:?} = {:?}", reg, reg_or_val);
        }
    }

    fn process(&mut self, mut instructions: Vec<Instruction>) {
        let mut instr_idx = 0i32;
        while instr_idx < instructions.len() as i32 {
            let instr = instructions[instr_idx as usize];
            match instr {
                Copy { source, target } => {
                    self.set_value(target, source);
                },
                Inc { reg } => {
                    if let Register(reg_idx) = reg {
                        self.regs[reg_idx] += 1;
                    }
                },
                Dec { reg } => {
                    if let Register(reg_idx) = reg {
                        self.regs[reg_idx] -= 1;
                    }
                },
                JumpNotZero { check, delta } => {
                    let value = self.value(check);
                    if value != 0 {
                        instr_idx += self.value(delta);
                        continue;
                    }
                },
                Toggle { reg } => {
                    let idx = instr_idx + self.value(reg);
                    if idx >= 0 && (idx as usize) < instructions.len() {
                        let toggled = instructions[idx as usize].toggle();

                        instructions[idx as usize] = toggled;
                    }
                }
            }

            instr_idx += 1;
        }
    }
}

fn main() {
    let instructions = parse(include_str!("input.txt"));

    let mut cpu = Cpu::new(7);
    cpu.process(instructions.clone());
    println!("{:?}", cpu.regs);

    let mut cpu2 = Cpu::new(12);
    cpu2.process(instructions);
    println!("{:?}", cpu2.regs);
}