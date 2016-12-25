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

struct Cpu<T> {
    regs: [i32; 4],
    exec_env: Box<ExecutionEnvironment<T>>
}

impl<T> Cpu<T> {
    fn new(a_val: i32, mut exec_env: Box<ExecutionEnvironment<T>>) -> Cpu<T> {
        exec_env.handle_output(0);
        Cpu {
            regs: [a_val, 0, 0, 0],
            exec_env: exec_env
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

    fn process(&mut self, mut unoptimised_instructions: Vec<Instruction>) -> Option<T> {
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
                Out { operand } => {
//                    print!("{}", self.value(operand));
                    let val = self.value(operand);
                    self.exec_env.handle_output(val);
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

            if let Some(reason) = self.exec_env.should_terminate(self.regs, instr_idx as usize) {
                return Some(reason);
            }
        }

        None
    }
}

fn parse(file: &str) -> Vec<Instruction> {
    file.lines().map(|line| Instruction::parse(line)).collect()
}

trait ExecutionEnvironment<T> {
    fn handle_output(&mut self, val: i32);
    fn should_terminate(&mut self, registers: [i32; 4], program_counter: usize) -> Option<T>;
}

enum ClockSeekingTermType {
    NotAClock,
    Clock
}

struct ClockSeekingExecEnv {
    prev_out: (Option<i32>, Option<i32>),
    out_count: usize,
    warmup_window: usize,
    trial_window: usize
}

impl ClockSeekingExecEnv {
    fn new(warmup_window:usize, trial_window:usize) -> ClockSeekingExecEnv {
        ClockSeekingExecEnv {
            prev_out: (None, None),
            out_count: 0,
            warmup_window: warmup_window,
            trial_window: trial_window
        }
    }
}

impl ExecutionEnvironment<ClockSeekingTermType> for ClockSeekingExecEnv {
    fn handle_output(&mut self, val: i32) {
        self.prev_out = (self.prev_out.1, Some(val));
        self.out_count += 1;
    }

    fn should_terminate(&mut self, _: [i32; 4], _: usize) -> Option<ClockSeekingTermType> {
        if self.out_count < self.warmup_window {
            // We're still warming up; don't terminate
            return None
        }
        if self.out_count >= self.warmup_window + self.trial_window {
            // If we saw enough outputs without an error, it's probably a clock signal
            return Some(ClockSeekingTermType::Clock)
        }
        if let (Some(a), Some(b)) = self.prev_out {
            if !((a == 0 && b == 1) || (a == 1 && b == 0)) {
                // Terminate if we're not in an alternating 0 / 1 sequence
                return Some(ClockSeekingTermType::NotAClock)
            }
        }

        // We've not decided either way yet, so don't terminate
        None
    }
}

fn main() {
    let instructions = parse(include_str!("input.txt"));

    for initial_a in 0.. {
        let exec_env = ClockSeekingExecEnv::new(10, 32); // Skip 1st 10, then examine 32 (& hope that's enough)
        let mut cpu = Cpu::new(initial_a, Box::new(exec_env));
        if let Some(ClockSeekingTermType::Clock) = cpu.process(instructions.clone()) {
            println!("Found somethign that looks like a clock with initial a value: {}", initial_a);
            break;
        }
    }
}