extern crate regex;
use regex::Regex;

use std::collections::VecDeque;
use std::collections::HashMap;
use std::iter::FromIterator;

fn scramble(plaintext: &str, file: &str) -> String {
    let swap_pos_re = Regex::new(r"swap position (?P<x>.+) with position (?P<y>.+)").unwrap();
    let swap_letter_re = Regex::new(r"swap letter (?P<x>.+) with letter (?P<y>.+)").unwrap();
    let rotate_steps_re = Regex::new(r"rotate (?P<dir>left|right) (?P<x>.+) step").unwrap();
    let rotate_pos_re = Regex::new(r"rotate based on position of letter (?P<x>.+)").unwrap();
    let reverse_pos_re = Regex::new(r"reverse positions (?P<x>.+) through (?P<y>.+)").unwrap();
    let move_pos_re = Regex::new(r"move position (?P<x>.+) to position (?P<y>.+)").unwrap();

    let mut chars = VecDeque::from_iter(plaintext.chars());

    for line in file.lines() {
        if let Some(caps) = swap_pos_re.captures(line) {
            //swap position X with position Y means that the letters at indexes X and Y (counting
            // from 0) should be swapped.
            let x = caps.name("x").unwrap().parse::<usize>().unwrap();
            let y = caps.name("y").unwrap().parse::<usize>().unwrap();
            let tmp = chars[x];
            chars[x] = chars[y];
            chars[y] = tmp;
        } else if let Some(caps) = swap_letter_re.captures(line) {
            //swap letter X with letter Y means that the letters X and Y should be swapped
            // (regardless of where they appear in the string).
            let x = caps.name("x").unwrap().chars().next().unwrap();
            let y = caps.name("y").unwrap().chars().next().unwrap();
            for c in &mut chars {
                if *c == x {
                    *c = y
                } else if *c == y {
                    *c = x
                }
            }
        } else if let Some(caps) = rotate_steps_re.captures(line) {
            //rotate left/right X steps means that the whole string should be rotated; for example,
            // one right rotation would turn abcd into dabc.
            let dir = caps.name("dir").unwrap();
            let x = caps.name("x").unwrap().parse::<usize>().unwrap() % chars.len();
            if dir == "right" {
                for _ in 0..x {
                    let c = chars.pop_back().unwrap();
                    chars.push_front(c);
                }
            } else {
                for _ in 0..x {
                    let c = chars.pop_front().unwrap();
                    chars.push_back(c);
                }
            }

        } else if let Some(caps) = rotate_pos_re.captures(line) {
            //rotate based on position of letter X means that the whole string should be rotated to
            // the right based on the index of letter X (counting from 0) as determined before this
            // instruction does any rotations. Once the index is determined, rotate the string to
            // the right one time, plus a number of times equal to that index, plus one additional
            // time if the index was at least 4.
            let x = caps.name("x").unwrap().chars().next().unwrap();
            let pos = chars.iter().position(|&c| c == x).unwrap();
            let num = (pos + if pos >=4 { 2 } else { 1 }) % chars.len();
            for _ in 0..num {
                let c = chars.pop_back().unwrap();
                chars.push_front(c);
            }
        } else if let Some(caps) = reverse_pos_re.captures(line) {
            //reverse positions X through Y means that the span of letters at indexes X through Y
            // (including the letters at X and Y) should be reversed in order.
            let x = caps.name("x").unwrap().parse::<usize>().unwrap();
            let y = caps.name("y").unwrap().parse::<usize>().unwrap();
            for i in 0..(y+1-x)/2 {
                let tmp = chars[x+i];
                chars[x+i] = chars[y-i];
                chars[y-i] = tmp;
            }
        } else if let Some(caps) = move_pos_re.captures(line) {
            //move position X to position Y means that the letter which is at index X should be
            // removed from the string, then inserted such that it ends up at index Y.
            let x = caps.name("x").unwrap().parse::<usize>().unwrap();
            let y = caps.name("y").unwrap().parse::<usize>().unwrap();
            let c = chars.remove(x).unwrap();
            chars.insert(y, c);
        } else {
            unreachable!("Could not understand instruction: {}", line);
        }

//        println!("{}: {:?}", line, chars);
    }

    chars.into_iter().collect()
}

fn unscramble(plaintext: &str, file: &str) -> String {
    let swap_pos_re = Regex::new(r"swap position (?P<x>.+) with position (?P<y>.+)").unwrap();
    let swap_letter_re = Regex::new(r"swap letter (?P<x>.+) with letter (?P<y>.+)").unwrap();
    let rotate_steps_re = Regex::new(r"rotate (?P<dir>left|right) (?P<x>.+) step").unwrap();
    let rotate_pos_re = Regex::new(r"rotate based on position of letter (?P<x>.+)").unwrap();
    let reverse_pos_re = Regex::new(r"reverse positions (?P<x>.+) through (?P<y>.+)").unwrap();
    let move_pos_re = Regex::new(r"move position (?P<x>.+) to position (?P<y>.+)").unwrap();

    // Pre-calculate inverse lookup of rotate positions
    let mut rotate_inverse_lookup = HashMap::new();
    let positions = (0..plaintext.len()).into_iter()
        .map(|i| (i + i + if i >=4 { 2 } else { 1 }) % plaintext.len());
    for (i, pos) in positions.enumerate() {
//        println!("({}, {})", i, pos);
        rotate_inverse_lookup.insert(pos, i);
    }

    let mut chars = VecDeque::from_iter(plaintext.chars());

    let mut lines = file.lines().collect::<Vec<_>>();
    lines.reverse();
    for line in lines {
        if let Some(caps) = swap_pos_re.captures(line) {
            //swap position X with position Y means that the letters at indexes X and Y (counting
            // from 0) should be swapped.
            let x = caps.name("x").unwrap().parse::<usize>().unwrap();
            let y = caps.name("y").unwrap().parse::<usize>().unwrap();
            let tmp = chars[x];
            chars[x] = chars[y];
            chars[y] = tmp;
        } else if let Some(caps) = swap_letter_re.captures(line) {
            //swap letter X with letter Y means that the letters X and Y should be swapped
            // (regardless of where they appear in the string).
            let x = caps.name("x").unwrap().chars().next().unwrap();
            let y = caps.name("y").unwrap().chars().next().unwrap();
            for c in &mut chars {
                if *c == x {
                    *c = y
                } else if *c == y {
                    *c = x
                }
            }
        } else if let Some(caps) = rotate_steps_re.captures(line) {
            //rotate left/right X steps means that the whole string should be rotated; for example,
            // one right rotation would turn abcd into dabc.
            let dir = caps.name("dir").unwrap();
            let x = caps.name("x").unwrap().parse::<usize>().unwrap() % chars.len();
            if dir != "right" {
                for _ in 0..x {
                    let c = chars.pop_back().unwrap();
                    chars.push_front(c);
                }
            } else {
                for _ in 0..x {
                    let c = chars.pop_front().unwrap();
                    chars.push_back(c);
                }
            }

        } else if let Some(caps) = rotate_pos_re.captures(line) {
            //rotate based on position of letter X means that the whole string should be rotated to
            // the right based on the index of letter X (counting from 0) as determined before this
            // instruction does any rotations. Once the index is determined, rotate the string to
            // the right one time, plus a number of times equal to that index, plus one additional
            // time if the index was at least 4.
            let x = caps.name("x").unwrap().chars().next().unwrap();
            let pos = chars.iter().position(|&c| c == x).unwrap();
            let orig_pos = match rotate_inverse_lookup.get(&pos) {
                Some(n) => n,
                None => panic!("Could not find rotate inverse for {} in {:?}", pos, rotate_inverse_lookup)
            };
            let num = (chars.len() + pos - orig_pos) % chars.len();
            for _ in 0..num {
                let c = chars.pop_front().unwrap();
                chars.push_back(c);
            }
        } else if let Some(caps) = reverse_pos_re.captures(line) {
            //reverse positions X through Y means that the span of letters at indexes X through Y
            // (including the letters at X and Y) should be reversed in order.
            let x = caps.name("x").unwrap().parse::<usize>().unwrap();
            let y = caps.name("y").unwrap().parse::<usize>().unwrap();
            for i in 0..(y+1-x)/2 {
                let tmp = chars[x+i];
                chars[x+i] = chars[y-i];
                chars[y-i] = tmp;
            }
        } else if let Some(caps) = move_pos_re.captures(line) {
            //move position X to position Y means that the letter which is at index X should be
            // removed from the string, then inserted such that it ends up at index Y.
            let x = caps.name("x").unwrap().parse::<usize>().unwrap();
            let y = caps.name("y").unwrap().parse::<usize>().unwrap();
            let c = chars.remove(y).unwrap();
            chars.insert(x, c);
        } else {
            unreachable!("Could not understand instruction: {}", line);
        }

//        println!("un{}: {:?}", line, chars);
    }

    chars.into_iter().collect()
}

fn main() {
    println!("Part 1: {}", scramble("abcdefgh", include_str!("input.txt")));
    println!("Part 2: {}", unscramble("fbgdceah", include_str!("input.txt")));
}

#[test]
fn abcde_test_scrambles_to_decab() {
    assert_eq!("decab", scramble("abcde", include_str!("input-test.txt")))
}

#[test]
fn decab_test_unscrambles_to_abcde() {
    assert_eq!("abcde", unscramble("decab", include_str!("input-test.txt")))
}

#[test]
fn bfheacgd_unscrambles_to_abcdefgh() {
    assert_eq!("abcdefgh", unscramble("bfheacgd", include_str!("input.txt")))
}