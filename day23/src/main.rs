extern crate regex;
use regex::Regex;

use std::collections::HashMap;

use Instruction::*;

#[derive(Debug, Clone, Copy)]
enum Instruction<'a> {
    CopyVal{value:i32, target:&'a str},
    CopyReg{source:&'a str, target:&'a str},
    Inc{reg:&'a str},
    Dec{reg:&'a str},
    JumpRegNotZero{check:&'a str, delta:&'a str},
    JumpValNotZero{value: i32, delta:&'a str},
    Toggle{reg:&'a str}
}

impl<'a> Instruction<'a> {
    fn toggle(&self) -> Instruction<'a> {
        match *self {
            CopyVal{value, target} => {
                JumpValNotZero{value: value, delta: target}
            },
            CopyReg{source, target} => {
                JumpRegNotZero{check: source, delta: target}
            },
            Inc{reg} => {
                Dec{reg: reg}
            },
            Dec{reg} => {
                Inc{reg: reg}
            },
            JumpValNotZero{value, delta} => {
                // string_to_static_str converts delta.to_string() into a &'static str, but
                // leaks the String memory! Probably CopyVal should take a String instead of
                // &str, but this is quicker for now...
                CopyVal{value: value, target: string_to_static_str(delta.to_string())}
            },
            JumpRegNotZero{check, delta} => {
                CopyReg{source: check, target: string_to_static_str(delta.to_string())}
            },
            Toggle{reg} => {
                Inc { reg: reg }
            }
        }
    }
}

use std::mem;

fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}

fn main() {
    let cpy_re:Regex = Regex::new(r"cpy (.+?) (.+)").unwrap();
    let inc_re = Regex::new(r"inc (.+?)").unwrap();
    let dec_re = Regex::new(r"dec (.+?)").unwrap();
    let jnz_re = Regex::new(r"jnz (.+?) (.+)").unwrap();
    let tgl_re = Regex::new(r"tgl (.+?)").unwrap();

    let file = include_str!("input.txt");
    let mut instructions = vec![];

    for line in file.lines() {
        let instr = if let Some(caps) = cpy_re.captures(line) {
            let val_or_reg = caps.at(1).unwrap();
            let target_reg = caps.at(2).unwrap();

            if let Ok(val) = val_or_reg.parse::<i32>() {
                CopyVal{value: val, target: target_reg}
            } else {
                CopyReg{source: val_or_reg, target: target_reg}
            }
        } else if let Some(caps) = inc_re.captures(line) {
            let reg = caps.at(1).unwrap();

            Inc{reg: reg}
        } else if let Some(caps) = dec_re.captures(line) {
            let reg = caps.at(1).unwrap();
            Dec{reg: reg}
        } else if let Some(caps) = jnz_re.captures(line) {
            let val_or_reg = caps.at(1).unwrap();
            let delta = caps.at(2).unwrap();

            if let Ok(val) = val_or_reg.parse::<i32>() {
                JumpValNotZero{value: val, delta: delta}
            } else {
                JumpRegNotZero{check: val_or_reg, delta: delta}
            }
        } else if let Some(caps) = tgl_re.captures(line) {
            let reg = caps.at(1).unwrap();
            Toggle{reg: reg}
        } else {
            unreachable!("Did not recognise instruction: {}", line);
        };

        instructions.push(instr);
    }

    let mut regs = HashMap::new();
    regs.insert("a", 7i32); // 7 for part 1, 12 for pqrt 2
    regs.insert("b", 0);
    regs.insert("c", 0);
    regs.insert("d", 0);

    let mut instr_idx = 0i32;
    while instr_idx < instructions.len() as i32 {
        let instr = instructions[instr_idx as usize];
        match instr {
            CopyVal { value, target } => {
                regs.insert(target, value);
//                println!("Inserting {} to {}", value, target);
            },
            CopyReg { source, target } => {
                let value = *regs.get(source).expect(&format!("Unknown register: '{}'", source));
                regs.insert(target, value);
//                println!("Inserting {} (from {}) to {}", value, source, target);
            },
            Inc { reg } => {
                let mut val = regs.get_mut(reg).unwrap();
                *val += 1;
//                println!("Incrementing {} to {}", reg, val);
            },
            Dec { reg } => {
                let mut val = regs.get_mut(reg).unwrap();
                *val -= 1;
//                println!("Decrementing {} to {}", reg, val);
            },
            JumpValNotZero { value, delta } => {
                let delta = if let Ok(d) = delta.parse::<i32>() {
                    d
                } else {
                    *regs.get(&delta).unwrap()
                };
                if value != 0 {
                    instr_idx += delta;
//                    println!("Jumping by {} to {}", delta, instr_idx);
                    continue;
                }
            },
            JumpRegNotZero { check, delta } => {
                let delta = if let Ok(d) = delta.parse::<i32>() {
                    d
                } else {
                    *regs.get(&delta).unwrap()
                };
                let value = *regs.get(check).expect(&format!("Unknown register: '{}'", check));
                if value != 0 {
                    instr_idx += delta;
//                    println!("Jumping by {} to {}", delta, instr_idx);
                    continue;
                }
            },
            Toggle { reg } => {
                let delta = *regs.get(reg).expect(&format!("Unknown register: '{}'", reg));
                let idx = instr_idx + delta;
                if idx >= 0 && (idx as usize) < instructions.len() {
                    let toggled = instructions[idx as usize].toggle();

                    instructions[idx as usize] = toggled;
                }
            }
        }

        instr_idx += 1;
//        println!("Tick! Counter: {} Regs: {:?}", instr_idx, regs);
    }
    println!("{:?}", regs);
}