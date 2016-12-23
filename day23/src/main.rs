#![feature(advanced_slice_patterns, slice_patterns)]

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
    Toggle{reg:Operand},
    MultiplyAddAndClear{factor_1:Operand, factor_2:Operand, target:Operand, clear:Operand},
    AddAndClear{source:Operand, target:Operand, clear:Operand},
    Nop
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
            },
            MultiplyAddAndClear{..} | AddAndClear{..} | Nop => {
                unreachable!("Trying to toggle an optimised instruction")
            }
        }
    }
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

fn optimise(instructions: &[Instruction]) -> Vec<Instruction> {
    let mut optimised = vec![];

    let mut windows = instructions.windows(6);
    while let Some(window) = windows.next() {
//        println!("Considering {:?}", window);
        if let &[
            Copy{source, target: Register(add_drain_cpy)},           // src -> drain
            Inc{reg: out},                                                   // drain -> out, 0 -> drain
            Dec{reg: Register(add_drain_dec)},                               // ...
            JumpNotZero{check: Register(add_drain_jmp), delta: Literal(-2)}, // ...
            Dec{reg: Register(multiply_drain_dec)},                               // repeat above mult_drain times, 0 -> mult_drain
            JumpNotZero{check: Register(multiply_drain_jmp), delta: Literal(-5)}, // ...
        ] = window {
            if add_drain_cpy == add_drain_dec && add_drain_dec == add_drain_jmp &&
                multiply_drain_dec == multiply_drain_jmp {
                let mult = MultiplyAddAndClear {
                    factor_1: source,
                    factor_2: Register(multiply_drain_dec),
                    target: out,
                    clear: Register(add_drain_cpy)
                };
                optimised.push(mult);
                optimised.push(Nop);
                optimised.push(Nop);
                optimised.push(Nop);
                optimised.push(Nop);
                optimised.push(Nop);

                // This instruction replaced six, so we need to ignore the next 6 windows to start
                // considering a fresh window.
                for _ in 0..5 {
                    windows.next();
                }
                continue;
            }
        } else if let &[
            Copy{source, target: Register(drain_cpy)},                   // src -> drain
            Dec{reg: Register(drain_dec)},                               // ...
            Inc{reg: out},                                               // drain -> out, 0 -> drain
            JumpNotZero{check: Register(drain_jmp), delta: Literal(-2)}, // ...
            _,
            _
        ] = window {
            if drain_cpy == drain_dec && drain_dec == drain_jmp {
                let add = AddAndClear{
                    source: source,
                    target: out,
                    clear: Register(drain_dec)
                };
                optimised.push(add);
                optimised.push(Nop);
                optimised.push(Nop);
                optimised.push(Nop);

                // This instruction replaced four, so we need to ignore the next 3 windows to start
                // considering a fresh window.
                for _ in 0..3 {
                    windows.next();
                }
                continue;
            }
        }

        optimised.push(window[0]);
    }

    let mut windows = instructions[optimised.len()..].windows(4);
    while let Some(window) = windows.next() {
//        println!("Considering {:?}", window);
        if let &[
            Copy{source, target: Register(drain_cpy)},                   // src -> drain
            Dec{reg: Register(drain_dec)},                               // ...
            Inc{reg: out},                                               // drain -> out, 0 -> drain
            JumpNotZero{check: Register(drain_jmp), delta: Literal(-2)}, // ...
        ] = window {
            if drain_cpy == drain_dec && drain_dec == drain_jmp {
                let add = AddAndClear{
                    source: source,
                    target: out,
                    clear: Register(drain_dec)
                };
                optimised.push(add);
                optimised.push(Nop);
                optimised.push(Nop);
                optimised.push(Nop);

                // This instruction replaced four, so we need to ignore the next 3 windows to start
                // considering a fresh window.
                for _ in 0..3 {
                    windows.next();
                }
                continue;
            }
        }

        optimised.push(window[0]);
    }

    for inst in &instructions[optimised.len()..] {
//        println!("Considering {:?}", inst);
        optimised.push(*inst);
    }

    optimised
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