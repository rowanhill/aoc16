//Disc #1 has 13 positions; at time=0, it is at position 1.
//Disc #2 has 19 positions; at time=0, it is at position 10.
//Disc #3 has 3 positions; at time=0, it is at position 2.
//Disc #4 has 7 positions; at time=0, it is at position 1.
//Disc #5 has 5 positions; at time=0, it is at position 3.
//Disc #6 has 17 positions; at time=0, it is at position 5.

struct Disc {
    positions: usize,
    delayed_start_pos: usize
}

impl Disc {
    fn new(positions: usize, start_pos:usize, drop_delay:usize) -> Disc {
        Disc {
            positions: positions,
            delayed_start_pos: start_pos + drop_delay
        }
    }

    fn pos_at_time(&self, time: usize) -> usize {
        (self.delayed_start_pos + time) % self.positions
    }

    fn combine_with(&self, other:Disc) -> Disc {
        Disc {
            positions: self.positions * other.positions,
            delayed_start_pos: 0
        }
    }
}

fn main() {
    let mut discs = vec![
        Disc::new(13, 1, 1),
        Disc::new(19, 10, 2),
        Disc::new(3, 2, 3),
        Disc::new(7, 1, 4),
        Disc::new(5, 3, 5),
        Disc::new(17, 5, 6),
        Disc::new(11, 0, 7), // Comment out for part 1
    ];

    let mut cur_discs = discs.remove(0);

    let mut time = cur_discs.positions - cur_discs.delayed_start_pos; // start with 1st disc as pos 0

    let mut count = 0;
    while discs.len() > 0 {
        let mut delta = 0;
        let next_disc = discs.remove(0);
        loop {
            count += 1;
            if next_disc.pos_at_time(time + delta) == 0 {
                time += delta;
                cur_discs = cur_discs.combine_with(next_disc);
                break;
            } else {
                delta += cur_discs.positions;
            }
        }
    }

    println!("Push the button at time {} (found in {} iterations)", time, count);
}
