extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

use std::collections::VecDeque;

const HEIGHT:usize = 4;
const WIDTH:usize = 4;

#[derive(Debug, PartialEq)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn from_index(i: usize) -> Dir {
        match i {
            0 => Dir::Up,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Right,
            _ => unreachable!()
        }
    }

    fn letter(&self) -> &str {
        match *self {
            Dir::Up => "U",
            Dir::Down => "D",
            Dir::Left => "L",
            Dir::Right => "R"
        }
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position {
            x: x,
            y: y
        }
    }

    fn can_go(&self, dir: &Dir) -> bool {
        !((self.x == 0 && *dir == Dir::Left) ||
              (self.x == WIDTH - 1 && *dir == Dir::Right) ||
              (self.y == 0 && *dir == Dir::Up) ||
              (self.y == HEIGHT - 1 && *dir == Dir::Down))
    }

    fn move_one(&self, dir: &Dir) -> Position {
        match *dir {
            Dir::Up => Position{ x: self.x, y: self.y - 1},
            Dir::Down => Position{ x: self.x, y: self.y + 1},
            Dir::Left => Position{ x: self.x - 1, y: self.y},
            Dir::Right => Position{ x: self.x + 1, y: self.y}
        }
    }

    fn is_vault(&self) -> bool {
        self.x == WIDTH - 1 && self.y == HEIGHT - 1
    }
}

fn get_unlocked_doors(path: &str, position: &Position) -> Vec<Dir> {
    let mut md5 = Md5::new();
    md5.input_str(path);
    let mut hash = md5.result_str();
    hash.truncate(4);

    let mut dirs = vec![];
    for (i, c) in hash.chars().enumerate() {
        match c {
            'b'...'f' => {
                let dir = Dir::from_index(i);
                if position.can_go(&dir) {
                    dirs.push(dir);
                }
            },
            _ => {}
        }
    }

    dirs
}

fn solve(passcode: String, position: Position) -> Option<(String, usize)> {
    let mut queue = VecDeque::new();
    queue.push_back((passcode.clone(), position));

    let mut shortest = None;
    let mut longest = None;

    while let Some((path, position)) = queue.pop_front() {
        for dir in get_unlocked_doors(&path, &position) {
            let new_path = path.to_string() + dir.letter();
            let new_pos = position.move_one(&dir);

            if new_pos.is_vault() {
                longest = Some(new_path.len() - passcode.len());
                if shortest.is_none() {
                    shortest = Some(new_path[passcode.len()..].to_string());
                }
            } else {
                queue.push_back((new_path, new_pos));
            }
        }
    };

    shortest.and_then(|s| longest.map(|l| (s, l)))
}

fn main() {
    let (shortest_path, longest_length) = solve("pvhmgsws".to_string(), Position::new(0,0)).unwrap();
    println!("Shortest path: {}, longest length: {}", shortest_path, longest_length);
}

#[test]
fn hijkl_has_no_solutions() {
    assert_eq!(solve("hijkl".to_string(), Position::new(0,0)), None);
}

#[test]
fn ihgpwlah() {
    assert_eq!(
    solve("ihgpwlah".to_string(), Position::new(0,0)),
    Some(("DDRRRD".to_string(), 370))
    );
}

#[test]
fn kglvqrro() {
    assert_eq!(
    solve("kglvqrro".to_string(), Position::new(0,0)),
    Some(("DDUDRLRRUDRD".to_string(), 492))
    );
}

#[test]
fn ulqzkmiv() {
    assert_eq!(
    solve("ulqzkmiv".to_string(), Position::new(0,0)),
    Some(("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string(), 830))
    );
}
