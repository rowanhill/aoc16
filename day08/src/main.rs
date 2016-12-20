extern crate regex;

use regex::Regex;

const WIDTH:usize = 50;
const HEIGHT:usize = 6;

#[derive(Debug)]
enum Instruction {
    Rect{width: usize, height: usize},
    RotateRow{index: usize, shift: usize},
    RotateCol{index: usize, shift: usize}
}

fn main() {
    let rect_re = Regex::new(r"rect (?P<width>\d+)x(?P<height>\d+)").unwrap();
    let rotate_row_re = Regex::new(r"rotate row y=(?P<index>\d+) by (?P<shift>\d+)").unwrap();
    let rotate_col_re = Regex::new(r"rotate column x=(?P<index>\d+) by (?P<shift>\d+)").unwrap();

    let instrs = include_str!("input.txt");

    let mut screen = vec![];
    for _ in 0..HEIGHT {
        let mut row = vec![];
        for _ in 0..WIDTH {
            row.push(false);
        }
        screen.push(row);
    }

    for instr_str in instrs.lines() {
        let instr = if rect_re.is_match(instr_str) {
            let caps = rect_re.captures(instr_str).unwrap();
            Instruction::Rect {
                width: caps.name("width").unwrap().parse::<usize>().unwrap(),
                height: caps.name("height").unwrap().parse::<usize>().unwrap()
            }
        } else if rotate_row_re.is_match(instr_str) {
            let caps = rotate_row_re.captures(instr_str).unwrap();
            Instruction::RotateRow {
                index: caps.name("index").unwrap().parse::<usize>().unwrap(),
                shift: caps.name("shift").unwrap().parse::<usize>().unwrap() % 50
            }
        } else if rotate_col_re.is_match(instr_str) {
            let caps = rotate_col_re.captures(instr_str).unwrap();
            Instruction::RotateCol {
                index: caps.name("index").unwrap().parse::<usize>().unwrap(),
                shift: caps.name("shift").unwrap().parse::<usize>().unwrap() % 6
            }
        } else {
            println!("{}", instr_str);
            unreachable!();
        };

        match instr {
            Instruction::Rect{width, height} => {
                for row in 0..height {
                    for col in 0..width {
                        screen[row][col] = true;
                    }
                }
            },
            Instruction::RotateRow{index, shift} => {
                let count_before = count(&screen);

                {
                    let ref mut row = screen[index];
                    for _ in 0..shift {
                        let pixel = row.pop().unwrap();
                        row.insert(0, pixel);
                    }
                }

                let count_after = count(&screen);
                if count_before != count_after {
                    panic!("Count changed rotating row, from {} to {}", count_before, count_after);
                }
            },
            Instruction::RotateCol{index, shift} => {
                let count_before = count(&screen);

                let mut clone_col = vec![];
                for row in 0..HEIGHT {
                    clone_col.push(screen[row][index]);
                }
                for row in 0..HEIGHT {
                    //                    println!("Move {} ({}) to {} ({})", row, clone_col[row], (row + shift) % HEIGHT, screen[(row + shift) % HEIGHT][index]);
                    screen[(row + shift) % HEIGHT][index] = clone_col[row]
                }

                let count_after = count(&screen);
                if count_before != count_after {
                    panic!("Count changed rotating col, from {} to {}", count_before, count_after);
                }
            }
        }

        println!("{:?}", instr);
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                print!("{}", if screen[row][col] { '#' } else { '.' });
            }
            println!("");
        }
        println!("");
    }

    let count = count(&screen);
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            print!("{}", if screen[row][col] { '#' } else { '.' });
        }
        println!("");
    }
    println!("");

    println!("Final count: {}", count);
}

fn count(screen: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if screen[row][col] {
                count += 1;
            }
        }
    }
    count
}