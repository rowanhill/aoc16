#![feature(advanced_slice_patterns, slice_patterns)]
#![feature(associated_consts)]

extern crate regex;

#[macro_use]
extern crate lazy_static;

mod parser;
mod optimiser;

use ::parser::Instruction;
use ::parser::Instruction::*;
use ::parser::Operand;
use ::parser::Operand::*;

use optimiser::optimise;

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

    fn process(&mut self, mut unoptimised_instructions: Vec<Instruction>) {
        let mut instructions = optimise(&unoptimised_instructions);

        let mut instr_idx = 0i32;
        while instr_idx < instructions.len() as i32 {
            let instr = instructions[instr_idx as usize];
//            print!("{}: ", instr_idx);
            match instr {
                Copy { source, target } => {
//                    print!("Setting {:?} to {:?}", target, source);
                    self.set_value(target, source);
                },
                Inc { reg } => {
                    if let Register(reg_idx) = reg {
//                        print!("Incrementing {:?}", reg);
                        self.regs[reg_idx] += 1;
                    }
                },
                Dec { reg } => {
                    if let Register(reg_idx) = reg {
//                        print!("Decrementing {:?}", reg);
                        self.regs[reg_idx] -= 1;
                    }
                },
                JumpNotZero { check, delta } => {
//                    print!("Jumping by {:?} if {:?} is zero", delta, check);
                    let value = self.value(check);
                    if value != 0 {
                        instr_idx += self.value(delta);

//                        println!("  {:?}", self.regs);
                        continue;
                    }
                },
                Toggle { reg } => {
                    let idx = instr_idx + self.value(reg);
//                    print!("Toggling #{:?}", idx);
                    if idx >= 0 && (idx as usize) < instructions.len() {
                        // Toggle the un-optimised instructions
                        let orig = unoptimised_instructions[idx as usize];
                        let toggled = orig.toggle();
                        unoptimised_instructions[idx as usize] = toggled;

                        // Re-optimise the instructions
                        instructions = optimise(&unoptimised_instructions);

//                        print!(". Toggled {:?} to {:?}, optimised to {:?}", orig, toggled, instructions[idx as usize]);
                    }
                },
                MultiplyAddAndClear{ factor_1, factor_2, target, clear } => {
//                    print!("Multiplying {:?} by {:?} and adding to {:?}, then clearing {:?} and {:?}", factor_1, factor_2, target, factor_2, clear);
                    if let Register(target_reg_idx) = target {
                        let value = self.value(factor_1) * self.value(factor_2);
                        self.regs[target_reg_idx] += value;
                        self.set_value(factor_2, Literal(0));
                    }
                    self.set_value(clear, Literal(0));

                    // Skip over the following Nops
                    instr_idx += 5;
                },
                AddAndClear{ source, target, clear } => {
//                    print!("Adding {:?} to {:?}, then clearing {:?}", source, target, source);
                    if let (Register(source_reg_idx), Register(target_reg_idx), Register(clear_reg_idx)) = (source, target, clear) {
                        self.regs[target_reg_idx] += self.regs[source_reg_idx];
                        self.regs[clear_reg_idx] = 0;
                    }

                    // Skip over the following Nops
                    instr_idx += 3;
                },
                Nop => {
                    panic!("Executed a Nop - we must have jumped here")
//                    print!("No-op");
                }
            }
            
//            println!("  {:?}", self.regs);

            instr_idx += 1;
        }
    }
}

fn parse(file: &str) -> Vec<Instruction> {
    file.lines().map(|line| Instruction::parse(line)).collect()
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

#[test]
fn optimise_replaces_add() {
    let file = r"cpy c d
dec d
inc c
jnz d -2
tgl c";
    let instructions = optimise(&parse(file));
    let matches = match &instructions[..] {
        &[AddAndClear{..}, Nop, Nop, Nop, Toggle{..}] => true,
        _ => false
    };
    assert!(matches, "Should be [AddAndClear, Nop, Nop, Nop, Toggle] but got {:?}", instructions);
}

#[test]
fn optimised_multiply_produces_same_result_as_normal() {
    let file = r"cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 96 c
jnz 79 d
inc a
inc d
jnz d -2
inc c
jnz c -5";
    let normal = parse(file);
    let optimised = optimise(&normal.clone());

    let mut cpu = Cpu::new(7);
    cpu.process(normal);

    let mut cpu2 = Cpu::new(7);
    cpu2.process(optimised);

    assert_eq!(cpu.regs, cpu2.regs);
}