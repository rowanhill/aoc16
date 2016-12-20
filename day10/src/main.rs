extern crate regex;
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Destination {
    Bot(usize),
    Output(usize)
}

#[derive(Copy, Clone)]
struct Move {
    value: usize,
    dest: Destination
}

impl Move {
    fn from_captures(caps:regex::Captures) -> Move {
        let val = caps.name("val").unwrap().parse::<usize>().unwrap();
        let bot = caps.name("bot").unwrap().parse::<usize>().unwrap();

        Move {
            value: val,
            dest: Destination::Bot(bot)
        }
    }

    fn new(val: usize, dest: Destination) -> Move {
        Move {
            value: val,
            dest: dest
        }
    }
}

struct Bot {
    id: usize,
    values: Vec<usize>,
    low_dest: Destination,
    high_dest: Destination
}

impl Bot {
    fn from_captures(caps:regex::Captures) -> (usize, Bot) {
        let cur_bot = caps.name("cur_bot").unwrap().parse::<usize>().unwrap();

        let low_type = caps.name("low_type").unwrap();
        let low_id = caps.name("low_id").unwrap().parse::<usize>().unwrap();
        let low_dest = if low_type == "bot" {
            Destination::Bot(low_id)
        } else if low_type == "output" {
            Destination::Output(low_id)
        } else {
            panic!("Unexpected destination type: {}", low_type)
        };

        let high_type = caps.name("high_type").unwrap();
        let high_id = caps.name("high_id").unwrap().parse::<usize>().unwrap();
        let high_dest = if high_type == "bot" {
            Destination::Bot(high_id)
        } else if high_type == "output" {
            Destination::Output(high_id)
        } else {
            panic!("Unexpected destination type: {}", high_type)
        };

        (cur_bot, Self::new(cur_bot, low_dest, high_dest))
    }

    fn new(id: usize, low_dest: Destination, high_dest: Destination) -> Bot {
        Bot {
            id: id,
            values: vec![],
            low_dest: low_dest,
            high_dest: high_dest
        }
    }

    fn add_value(&mut self, val: usize) -> Option<(Move, Move)> {
        if self.values.len() >= 2 {
            panic!("Tried to add value {} to bot {}, but already had values {:?}",
                   val, self.id, self.values);
        }

        self.values.push(val);

        if self.values.len() == 2 {
            let (low, high) = if self.values[0] < self.values[1] {
                (self.values[0], self.values[1])
            } else {
                (self.values[1], self.values[0])
            };

            Some((Move::new(low, self.low_dest), Move::new(high, self.high_dest)))
        } else {
            None
        }
    }
}

struct Factory {
    bots: HashMap<usize, Bot>,
    outputs: HashMap<usize, usize>,
    moves: Vec<Move>
}

impl Factory {
    fn new() -> Factory {
        Factory {
            bots: HashMap::new(),
            outputs: HashMap::new(),
            moves: vec![]
        }
    }

    fn parse_instructions(&mut self, file: &str) {
        let give_value_re = Regex::new(r"value (?P<val>\d+) goes to bot (?P<bot>\d+)").unwrap();
        let low_high_re:Regex = Regex::new(r"bot (?P<cur_bot>\d+) gives low to (?P<low_type>bot|output) (?P<low_id>\d+) and high to (?P<high_type>bot|output) (?P<high_id>\d+)").unwrap();

        for line in file.lines() {
            if let Some(caps) = low_high_re.captures(line) {
                let (id, bot) = Bot::from_captures(caps);
                self.bots.insert(id, bot);
            } else if let Some(caps) = give_value_re.captures(line) {
                self.moves.push(Move::from_captures(caps))
            } else {
                panic!("Unrecognised line: {}", line);
            }
        }
    }

    fn process_moves(&mut self, watch_for_low: usize, watch_for_high: usize) {
        while self.moves.len() > 0 {
            let m:Move = self.moves.remove(0);
            match m.dest {
                Destination::Bot(id) => {
                    let ref mut bot = self.bots.get_mut(&id).unwrap();
                    if let Some((m1, m2)) = bot.add_value(m.value) {
                        self.moves.push(m1);
                        self.moves.push(m2);

                        if m1.value == watch_for_low && m2.value == watch_for_high {
                            println!("Bot {} responsible for comparing {} & {}",
                                     id, watch_for_low, watch_for_high);
                        }
                    };
                },
                Destination::Output(id) => {
                    if let Some(old) = self.outputs.insert(id, m.value) {
                        panic!("Output {}: overwriting {} with {}", id, old, m.value)
                    };
                }
            }
        }
    }

    fn out(&self, i: usize) -> &usize {
        self.outputs.get(&i).unwrap()
    }

    fn multiply_outputs(&self, x: usize, y: usize, z: usize) -> usize {
        self.out(x) * self.out(y) * self.out(z)
    }
}

fn main() {
    println!("Example factory:");

    let test_file = include_str!("input-test.txt");

    let mut test_factory = Factory::new();
    test_factory.parse_instructions(test_file);

    test_factory.process_moves(2, 5);

    println!("Outputs 0, 1, 2: [{}, {}, {}]", test_factory.out(0), test_factory.out(1), test_factory.out(2));

    // --------

    println!("");
    println!("Real factory:");

    let file = include_str!("input.txt");

    let mut factory = Factory::new();
    factory.parse_instructions(file);

    factory.process_moves(17, 61);

    println!("Output 0 * 1 * 2 = {}", factory.multiply_outputs(0, 1, 2));
}
