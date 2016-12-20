enum Mode {
    Simple,
    Recursive
}

fn decompress(input: &str, mode: &Mode) -> usize {
    let mut expanded_len = 0;

    let mut current_slice = input;
    while let Some(instr_start) = current_slice.find('(') {
//        println!("{}", current_slice);
        // Record length up to start of instructions
        expanded_len += instr_start;

        // Read instructions
        let instr_end = current_slice.find(')').unwrap();
        let instr = &current_slice[instr_start+1..instr_end];
//        println!("{}", instr);

        // Decode instructions
        let parts:Vec<&str> = instr.split("x").collect();
        let encoded_run_len = parts[0].parse::<usize>().unwrap();
        let num_repeats = parts[1].parse::<usize>().unwrap();
//        println!("{} x {}", encoded_run_len, num_repeats);

        match *mode {
            Mode::Recursive => {
                // Recursively decompress
                let child_expanded_len = decompress(&current_slice[instr_end+1..(instr_end+1+encoded_run_len)], &mode);
                expanded_len += num_repeats * child_expanded_len;
            },
            Mode::Simple => {
                expanded_len += num_repeats * encoded_run_len;
            }
        }

        // Read past encoded part of string
        current_slice = &current_slice[(instr_end+1+encoded_run_len)..];
    }

    // Record length of unencoded tail of string
    expanded_len += current_slice.len();

    expanded_len
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", decompress(input, &Mode::Simple));
    println!("Part 2: {}", decompress(input, &Mode::Recursive));
}