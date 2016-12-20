extern crate regex;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
enum TurnDir { Right, Left }

#[derive(Debug)]
struct Instruction {
    turn: TurnDir,
    distance: i32
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum CompassDir {
    North,
    East,
    South,
    West
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
    orientation: CompassDir
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            orientation: CompassDir::North
        }
    }

    fn process(&self, instruction: &Instruction) -> Vec<Position> {
        let new_orientation = match self.orientation {
            CompassDir::North => match instruction.turn {
                TurnDir::Left => CompassDir::West,
                TurnDir::Right => CompassDir::East
            },
            CompassDir::East => match instruction.turn {
                TurnDir::Left => CompassDir::North,
                TurnDir::Right => CompassDir::South
            },
            CompassDir::South => match instruction.turn {
                TurnDir::Left => CompassDir::East,
                TurnDir::Right => CompassDir::West
            },
            CompassDir::West => match instruction.turn {
                TurnDir::Left => CompassDir::South,
                TurnDir::Right => CompassDir::North
            }
        };

        let (x_delta_f, y_delta_f) = match new_orientation {
            CompassDir::North => (0, 1),
            CompassDir::East => (1, 0),
            CompassDir::South => (0, -1),
            CompassDir::West => (-1, 0)
        };

        let mut positions = vec![];

        for dist in 0..instruction.distance {
            let pos = Position {
                x: self.x + (dist+1)*x_delta_f,
                y: self.y + (dist+1)*y_delta_f,
                orientation: new_orientation
            };
            positions.push(pos);
        }

        positions
    }
}

fn main() {
    let instr_str = "R1, R1, R3, R1, R1, L2, R5, L2, R5, R1, R4, L2, R3, L3, R4, L5, R4, R4, R1, L5, L4, R5, R3, L1, R4, R3, L2, L1, R3, L4, R3, L2, R5, R190, R3, R5, L5, L1, R54, L3, L4, L1, R4, R1, R3, L1, L1, R2, L2, R2, R5, L3, R4, R76, L3, R4, R191, R5, R5, L5, L4, L5, L3, R1, R3, R2, L2, L2, L4, L5, L4, R5, R4, R4, R2, R3, R4, L3, L2, R5, R3, L2, L1, R2, L3, R2, L1, L1, R1, L3, R5, L5, L1, L2, R5, R3, L3, R3, R5, R2, R5, R5, L5, L5, R2, L3, L5, L2, L1, R2, R2, L2, R2, L3, L2, R3, L5, R4, L4, L5, R3, L4, R1, R3, R2, R4, L2, L3, R2, L5, R5, R4, L2, R4, L1, L3, L1, L3, R1, R2, R1, L5, R5, R3, L3, L3, L2, R4, R2, L5, L1, L1, L5, L4, L1, L1, R1";
    //let instr_str = "R8, R4, R4, R8";
    let instr_splits:Vec<&str> = instr_str.split(", ").collect();

    let re:Regex = Regex::new(r"^(?P<dir>[RL])(?P<dist>\d+)$").unwrap();

    let instructions:Vec<Instruction> = instr_splits.into_iter().map(|i| {
        let captures:regex::Captures = re.captures(i).unwrap();
        Instruction {
            turn: match captures.name("dir").unwrap() {
                "R" => TurnDir::Right,
                "L" => TurnDir::Left,
                _ => panic!("unexpected turn dir")
            },
            distance: captures.name("dist").unwrap().parse::<i32>().unwrap()
        }
    }).collect();

    let mut has_repeated_pos = false;
    let mut prevs = HashSet::new();
    let mut pos = Position::new();
    prevs.insert(format!("({}, {})", pos.x, pos.y));

//    println!("(0, 0)");

    for instr in instructions {
        let new_positions:Vec<Position> = pos.process(&instr);

//        if !has_repeated_pos {
//            println!("{} {:?}", &instr.distance, new_positions[0].orientation);
//        }

        for p in new_positions {
            if !has_repeated_pos {
//                println!("({}, {})", p.x, p.y);

                let was_contained = !prevs.insert(format!("({}, {})", p.x, p.y));
                if was_contained {
                    println!("First repeated position: ({}, {}). Distance: {}", p.x, p.y, p.x.abs()+p.y.abs());
                    has_repeated_pos = true;
                }
            }

            pos = p;
        }
    }

    println!("Final position: ({}, {}). Distance: {}", pos.x, pos.y, pos.x.abs()+pos.y.abs());
}
