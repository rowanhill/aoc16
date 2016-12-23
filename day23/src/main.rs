extern crate regex;
use regex::Regex;

use Instruction::*;

#[derive(Debug, Clone, Copy)]
enum Instruction<'a> {
    Copy{source:&'a str, target:&'a str},
    Inc{reg:&'a str},
    Dec{reg:&'a str},
    JumpNotZero{check:&'a str, delta:&'a str},
    Toggle{reg:&'a str}
}

impl<'a> Instruction<'a> {
    fn toggle(&self) -> Instruction<'a> {
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
    let cpy_re:Regex = Regex::new(r"cpy (.+?) (.+)").unwrap();
    let inc_re = Regex::new(r"inc (.+?)").unwrap();
    let dec_re = Regex::new(r"dec (.+?)").unwrap();
    let jnz_re = Regex::new(r"jnz (.+?) (.+)").unwrap();
    let tgl_re = Regex::new(r"tgl (.+?)").unwrap();

    let mut instructions = vec![];

    for line in file.lines() {
        let instr = if let Some(caps) = cpy_re.captures(line) {
            let val_or_reg = caps.at(1).unwrap();
            let target_reg = caps.at(2).unwrap();
            Copy{source: val_or_reg, target: target_reg}
        } else if let Some(caps) = inc_re.captures(line) {
            let reg = caps.at(1).unwrap();
            Inc{reg: reg}
        } else if let Some(caps) = dec_re.captures(line) {
            let reg = caps.at(1).unwrap();
            Dec{reg: reg}
        } else if let Some(caps) = jnz_re.captures(line) {
            let val_or_reg = caps.at(1).unwrap();
            let delta = caps.at(2).unwrap();
            JumpNotZero{check: val_or_reg, delta: delta}
        } else if let Some(caps) = tgl_re.captures(line) {
            let reg = caps.at(1).unwrap();
            Toggle{reg: reg}
        } else {
            unreachable!("Did not recognise instruction: {}", line);
        };

        instructions.push(instr);
    }

    instructions
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

    fn reg_idx(reg: &str) -> usize {
        match reg {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            _   => unreachable!("Unexpected register name: {}", reg)
        }
    }

    fn value(&self, reg_or_val: &str) -> i32 {
        match reg_or_val {
            "a" | "b" | "c" | "d" => self.regs[Self::reg_idx(reg_or_val)],
            _ => reg_or_val.parse::<i32>().unwrap()
        }
    }

    fn set_value(&mut self, reg: &str, reg_or_val: &str) {
        let value = self.value(reg_or_val);
        self.regs[Self::reg_idx(reg)] = value;
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
                    self.regs[Self::reg_idx(reg)] = self.value(reg) + 1;
                },
                Dec { reg } => {
                    self.regs[Self::reg_idx(reg)] = self.value(reg) - 1;
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