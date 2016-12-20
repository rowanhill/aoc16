fn naive_part2() {
    let mut elves = vec![];
    for i in 0..3005290 {
        elves.push(i+1);
    }

    let mut current = 0;
    while elves.len() > 1 {
//        println!("{}:{}, {:?}", current, elves[current], elves);
        let to_kill = ((elves.len() / 2) + current) % elves.len();
        elves.remove(to_kill);
        if current < to_kill { current += 1}
        current = current % elves.len();
    }

    println!("{}", elves[0]);
}

struct ElfCircle {
    elves: [bool; 3005290],
    size: usize,
    to_remove: usize
}

impl ElfCircle {
    fn new() -> ElfCircle {
        ElfCircle {
            elves: [true; 3005290],
            size: 3005290,
            to_remove: 3005290 / 2
        }
    }

    fn remove_opposite(&mut self) {
        self.elves[self.to_remove] = false;

        let skip = if self.size % 2 == 0 { 1 } else { 2 };

        self.size -= 1;

        for _ in 0..skip {
            self.move_on();
        }
    }

    fn move_on(&mut self) {
        self.to_remove = (self.to_remove + 1) % self.elves.len();
        while !self.elves[self.to_remove] {
            self.to_remove = (self.to_remove + 1) % self.elves.len();
        }
    }

    fn remove_all_but_one(&mut self) -> usize {
        while self.size > 1 {
            self.remove_opposite();
        }

        self.elves.into_iter().position(|&b| b).unwrap()
    }
}

// The survivor for a given number of elves is a series that follows a pattern, wrapping around at
// each power of three:
// - each n from 3^x to 2 * 3^x (inclusive) => n - 3^x
// - each n from 2 * 3^x + 1 to 3^(x+1) - 1 => (n - 3^x) + (n - 3^x - 3^x)
// so it goes:
//  1 => 1
//
//  2 => 1
//  3 => 2 + 1
//
//  4 => 1
//  5 => 2
//  6 => 3
//  7 => 4 + 1
//  8 => 5 + 2
//  9 => 6 + 3
fn find_survivor_by_pattern(num_elves: usize) -> usize {
    let mut biggest_smaller_power = 1;
    while biggest_smaller_power*3 < num_elves {
        biggest_smaller_power *= 3;
    }
    num_elves - biggest_smaller_power + std::cmp::max(num_elves - 2*biggest_smaller_power, 0)
}

fn main() {
    // Part 1: Josephus problem: subtract highest power of 2, double, add one.

    // Part 2:
    // Originally solved naively, by simulating removing elves from a vec.
    naive_part2();

    // Better solution: simulating by array of bools
    let mut ring = ElfCircle::new();
    println!("{}", ring.remove_all_but_one());

    // Even better solution (having read around): numerically calculate survivor based on pattern
    println!("{}", find_survivor_by_pattern(3005290));
}
