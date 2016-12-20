extern crate regex;
use regex::Regex;

use std::collections::HashMap;

use Instruction::*;

enum Instruction<'a, 'b> {
    CopyVal{value:i32, target:&'a str},
    CopyReg{source:&'a str, target:&'b str},
    Inc{reg:&'a str},
    Dec{reg:&'a str},
    JumpRegNotZero{check:&'a str, delta:i32},
    JumpValNotZero{value: i32, delta:i32}
}

fn main() {
    let cpy_re:Regex = Regex::new(r"cpy (.+?) (.+)").unwrap();
    let inc_re = Regex::new(r"inc (.+?)").unwrap();
    let dec_re = Regex::new(r"dec (.+?)").unwrap();
    let jnz_re = Regex::new(r"jnz (.+?) (.+)").unwrap();

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
            let delta = caps.at(2).unwrap().parse::<i32>().unwrap();

            if let Ok(val) = val_or_reg.parse::<i32>() {
                JumpValNotZero{value: val, delta: delta}
            } else {
                JumpRegNotZero{check: val_or_reg, delta: delta}
            }
        } else {
            unreachable!("Did not recognise instruction: {}", line);
        };

        instructions.push(instr);
    }

    let mut regs = HashMap::new();
    regs.insert("a", 0i32);
    regs.insert("b", 0);
    regs.insert("c", 1); // Set to 0 for part 1
    regs.insert("d", 0);

    let mut instr_idx = 0i32;
    while instr_idx < instructions.len() as i32 {
        let ref instr = instructions[instr_idx as usize];
        match *instr {
            CopyVal{value, target} => {
                regs.insert(target, value);
//                println!("Inserting {} to {}", value, target);
            },
            CopyReg{source, target} => {
                let value = *regs.get(source).expect(&format!("Unknown register: '{}'", source));
                regs.insert(target, value);
//                println!("Inserting {} (from {}) to {}", value, source, target);
            },
            Inc{reg} => {
                let mut val = regs.get_mut(reg).unwrap();
                *val += 1;
//                println!("Incrementing {} to {}", reg, val);
            },
            Dec{reg} => {
                let mut val = regs.get_mut(reg).unwrap();
                *val -= 1;
//                println!("Decrementing {} to {}", reg, val);
            },
            JumpValNotZero{value, delta} => {
                if value != 0 {
                    instr_idx += delta;
//                    println!("Jumping by {} to {}", delta, instr_idx);
                    continue;
                }
            },
            JumpRegNotZero{check, delta} => {
                let value = *regs.get(check).expect(&format!("Unknown register: '{}'", check));
                if value != 0 {
                    instr_idx += delta;
//                    println!("Jumping by {} to {}", delta, instr_idx);
                    continue;
                }
            }
        }

        instr_idx += 1;
//        println!("Tick! Counter: {} Regs: {:?}", instr_idx, regs);
    }

    println!("{:?}", regs);
}