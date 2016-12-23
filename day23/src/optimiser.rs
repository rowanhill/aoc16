use ::parser::Instruction;
use ::parser::Instruction::*;
use ::parser::Operand::*;

trait Optimisation {
    const LENGTH: usize;

    fn optimise(&self, window: &[Instruction]) -> Option<Instruction>;
}

// Multiply and Add current only match instructions in the window if the order is as expected. In
// fact, some instructions can be re-ordered without any semantic difference, so those orderings
// should also be recognised as optimisable.
// In practice, the current implementation is fine for the AoC puzzle input, but a more general
// solution would be fun.
//
// One possible option is to do some data flow analysis, and look for patterns in the data flow
// graph, rather than the original instruction list. See data-flow.txt for some thoughts in that
// direction.

struct Multiply {}
impl Optimisation for Multiply {
    const LENGTH: usize = 6;

    fn optimise(&self, window: &[Instruction]) -> Option<Instruction> {
        if window.len() < Self::LENGTH {
            return None;
        }

        // If the window has the right instructions in the right order...
        if let &[
            Copy{source, target: Register(add_drain_cpy)},                        // => src -> drain
            Inc{reg: out},                                                        // => drain -> out, 0 -> drain
            Dec{reg: Register(add_drain_dec)},                                    // |
            JumpNotZero{check: Register(add_drain_jmp), delta: Literal(-2)},      // |
            Dec{reg: Register(multiply_drain_dec)},                               // => repeat above mult_drain times, 0 -> mult_drain
            JumpNotZero{check: Register(multiply_drain_jmp), delta: Literal(-5)}, // |
        ] = &window[0..Self::LENGTH] {

            // ...and the references registers follow the right pattern...
            if add_drain_cpy == add_drain_dec && add_drain_dec == add_drain_jmp &&
                multiply_drain_dec == multiply_drain_jmp {

                // ...then we can optimise
                return Some(MultiplyAddAndClear {
                    factor_1: source,
                    factor_2: Register(multiply_drain_dec),
                    target: out,
                    clear: Register(add_drain_cpy)
                });
            }
        }

        return None;
    }
}

struct Add {}
impl Optimisation for Add {
    const LENGTH: usize = 4;

    fn optimise(&self, window: &[Instruction]) -> Option<Instruction> {
        if window.len() < Self::LENGTH {
            return None;
        }

        // If the window has the right instructions in the right order...
        if let &[
            Copy{source, target: Register(drain_cpy)},                   // => src -> drain
            Dec{reg: Register(drain_dec)},                               // |
            Inc{reg: out},                                               // => drain -> out, 0 -> drain
            JumpNotZero{check: Register(drain_jmp), delta: Literal(-2)}, // |
        ] = &window[0..Self::LENGTH] {

            // ...and the references registers follow the right pattern...
            if drain_cpy == drain_dec && drain_dec == drain_jmp {

                // ...then we can optimise
                return Some(AddAndClear{
                    source: source,
                    target: out,
                    clear: Register(drain_dec)
                });
            }
        }

        return None;
    }
}

fn run_optimisation<T: Optimisation>(
    optimisation: &T,
    optimised: &mut Vec<Instruction>,
    window: &[Instruction],
    windows: &mut ::std::slice::Windows<Instruction>
) -> bool {
    if let Some(mult) = optimisation.optimise(window) {
        optimised.push(mult);
        for _ in 0..T::LENGTH-1 {
            optimised.push(Nop);
            windows.next();
        }
        true
    } else {
        false
    }
}

pub fn optimise(instructions: &[Instruction]) -> Vec<Instruction> {
    let mut optimised = vec![];

    let multiply = Multiply {};
    let add = Add {};

    // Consider 6-instr windows
    let mut windows = instructions.windows(6);
    while let Some(window) = windows.next() {
        //        println!("Considering {:?}", window);
        if run_optimisation(&multiply, &mut optimised, window, &mut windows) { continue; }
        if run_optimisation(&add, &mut optimised, window, &mut windows) { continue; }
        optimised.push(window[0]);
    }

    // consider final 4-wide windows
    let mut windows = instructions[optimised.len()..].windows(4);
    while let Some(window) = windows.next() {
        //        println!("Considering {:?}", window);
        if run_optimisation(&add, &mut optimised, window, &mut windows) { continue; }
        optimised.push(window[0]);
    }

    // consider final single instructions
    for inst in &instructions[optimised.len()..] {
        //        println!("Considering {:?}", inst);
        optimised.push(*inst);
    }

    optimised
}

#[test]
fn multiply_optimise_matches_instructions_which_multiply_two_registers() {
    // source: 0, add_drain: 1, out: 2, mult_drain: 3
    let instructions = [
        Copy{source: Register(0), target: Register(1)},
        Inc{reg: Register(2)},
        Dec{reg: Register(1)},
        JumpNotZero{check: Register(1), delta: Literal(-2)},
        Dec{reg: Register(3)},
        JumpNotZero{check: Register(3), delta: Literal(-5)}
    ];
    let result = Multiply::optimise(&instructions);
    assert!(result.is_some());
}

#[test]
fn optimise_matches_instructions_which_multiply_two_registers_and_adds_noops() {
    // source: 0, add_drain: 1, out: 2, mult_drain: 3
    let instructions = [
        Copy{source: Register(0), target: Register(1)},
        Inc{reg: Register(2)},
        Dec{reg: Register(1)},
        JumpNotZero{check: Register(1), delta: Literal(-2)},
        Dec{reg: Register(3)},
        JumpNotZero{check: Register(3), delta: Literal(-5)},
        Dec{reg: Register(5)}
    ];
    let optimised = optimise(&instructions);
    assert!(match &optimised[..] {
        &[MultiplyAddAndClear{..}, Nop, Nop, Nop, Nop, Nop, Dec{..}] => true,
        _ => false
    });
}