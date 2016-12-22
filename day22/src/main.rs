extern crate regex;
use regex::Regex;

#[macro_use] extern crate itertools;
use itertools::Itertools;

fn count_viable_pairs(file:&str) -> usize {
    let df_re = Regex::new(
        r"dev/grid/node-x(?P<x>\d+)-y(?P<y>\d+)\s+(?P<size>\d+)T\s+(?P<used>\d+)T\s+(?P<avail>\d+)T\s+(?P<use>\d+)%"
    ).unwrap();

    let mut nodes = vec![];
    for line in file.lines().skip(2) {
        if let Some(captures) = df_re.captures(line) {
            let x = captures.name("x").unwrap().parse::<usize>().unwrap();
            let y = captures.name("y").unwrap().parse::<usize>().unwrap();
            let avail = captures.name("avail").unwrap().parse::<usize>().unwrap();
            let used = captures.name("used").unwrap().parse::<usize>().unwrap();
            nodes.push(((x,y), avail, used));
        } else {
            unreachable!("Line in unexpected format: {}", line);
        }
    }
    nodes.sort_by_key(|&(_, avail, _)| avail);

    nodes.iter()
        .filter(|&&(_, _, used)| used > 0)
        .map(|&(_, _, used)| {
        let mut index = match nodes.binary_search_by_key(&used, |&(_, avail, _)| avail) {
            Ok(index) => index,
            Err(index) => index
        };
        while index > 1 && nodes[index - 1].1 >= used {
            index -= 1;
        }

        nodes.len() - std::cmp::max(1, index)
    }).sum()
}

#[derive(Debug)]
enum PieceType {
    Empty,
    Sliding,
    Fixed,
    Goal
}

#[derive(Debug)]
struct Piece {
    piece_type: PieceType,
    x: usize,
    y: usize
}

struct Board {
    pieces: Vec<Vec<Piece>>
}

impl Board {
    fn new(file: &str, size: usize) -> Board {
        let df_re = Regex::new(
            r"dev/grid/node-x(?P<x>\d+)-y(?P<y>\d+)\s+(?P<size>\d+)T\s+(?P<used>\d+)T\s+(?P<avail>\d+)T\s+(?P<use>\d+)%"
        ).unwrap();

        let mut nodes = vec![];
        for line in file.lines().skip(2) {
            if let Some(captures) = df_re.captures(line) {
                let x = captures.name("x").unwrap().parse::<usize>().unwrap();
                let y = captures.name("y").unwrap().parse::<usize>().unwrap();
                let avail = captures.name("avail").unwrap().parse::<usize>().unwrap();
                let used = captures.name("used").unwrap().parse::<usize>().unwrap();
                nodes.push(((x,y), avail, used));
            } else {
                unreachable!("Line in unexpected format: {}", line);
            }
        }
        nodes.sort_by_key(|&(_, avail, _)| avail);

        let mut pieces = nodes.iter()
            .map(|node| {
                let &((x, y), _, used) = node;
                let piece_type:PieceType = {
                    // The node is either empty, the goal data, or should have exactly one or zero
                    // viable pair: each node with a viable pair can only be moved into the single
                    // empty node in the grid.
                    // I.e. it's a sliding tile puzzle with some fixed elements

                    if used == 0 {
                        PieceType::Empty
                    } else if y == 0 && x == size {
                        PieceType::Goal
                    } else {
                        let mut index = match nodes.binary_search_by_key(&used, |&(_, avail, _)| avail) {
                            Ok(index) => index,
                            Err(index) => index
                        };
                        while index > 1 && nodes[index - 1].1 >= used {
                            index -= 1;
                        }
                        let count = nodes.len() - std::cmp::max(1, index);
                        match count {
                            0 => PieceType::Fixed,
                            1 => PieceType::Sliding,
                            n => unreachable!("Unexpected number of viable pairs: {}", n)
                        }
                    }
                };
                Piece {
                    piece_type: piece_type,
                    x: x,
                    y: y
                }
            })
            .collect::<Vec<Piece>>();
        pieces.sort_by_key(|piece| piece.y);
        let grouped_pieces:Vec<Vec<Piece>> = pieces
            .into_iter()
            .group_by(|piece| piece.y).into_iter()
            .map(|(_, group)| {
                let mut row = group.collect::<Vec<_>>();
                row.sort_by_key(|piece| piece.x);
                row
            })
            .collect();

        Board {
            pieces: grouped_pieces
        }
    }

    fn visualise(&self) -> String {
        let mut result = String::new();
        for row in self.pieces.iter() {
            for piece in row.iter() {
                let p = match piece.piece_type {
                    PieceType::Empty => '_',
                    PieceType::Sliding => '.',
                    PieceType::Fixed => '#',
                    PieceType::Goal => 'G'
                };
                result.push(p);
            }
            result.push('\n');
        }
        result
    }
}

fn main() {
    let file = include_str!("input.txt");

    println!("Viable pairs: {}", count_viable_pairs(file));

    println!("");

    println!("Part 2 board:");
    let board = Board::new(file, 35);
    println!("{}", board.visualise());

    // Worked out part 2 by hand from here, as it's clear that the board is just a horizontal wall
    // of fixed pieces in the middle, and the empty space two rows up from the bottom right.
    // Steps taken:
    //  - move empty space up to wall: 9 moves [9 total]
    //  - move to left of wall: 34 moves [43 total]
    //  - move up to top row: 18 moves [61 total]
    //  - move to top right (shifting goal left one): 34 moves [95 total]
    //  - moving the goal left now takes 5 moves (empty space down one, left three, up one, goal one left)
    //  - We need to move it 34 times: 170 moves [265 total]
    // Therefore a minimum total of 265 moves needed
}

#[test]
fn one_smaller_than_all_others() {
    let file = r"root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y0     91T   66T    25T   72%
/dev/grid/node-x0-y1     87T    2T    20T   78%
/dev/grid/node-x0-y2     93T   73T    19T   78%
/dev/grid/node-x0-y3     89T   69T    20T   77%
/dev/grid/node-x0-y4     88T   67T    21T   76%
/dev/grid/node-x0-y5     87T   72T    15T   82%";
    let pairs = count_viable_pairs(file);
    assert_eq!(5, pairs);
}

#[test]
fn one_smaller_than_some() {
    let file = r"root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y0     91T   66T    25T   72%
/dev/grid/node-x0-y1     87T   21T    20T   78%
/dev/grid/node-x0-y2     93T   73T    19T   78%
/dev/grid/node-x0-y3     89T   69T    20T   77%
/dev/grid/node-x0-y4     88T   67T    21T   76%
/dev/grid/node-x0-y5     87T   72T    15T   82%";
    let pairs = count_viable_pairs(file);
    assert_eq!(2, pairs);
}

#[test]
fn none_fit() {
    let file = r"root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y0     91T   66T    25T   72%
/dev/grid/node-x0-y1     87T   80T    20T   78%
/dev/grid/node-x0-y2     93T   73T    19T   78%
/dev/grid/node-x0-y3     89T   69T    20T   77%
/dev/grid/node-x0-y4     88T   67T    21T   76%
/dev/grid/node-x0-y5     87T   72T    15T   82%";
    let pairs = count_viable_pairs(file);
    assert_eq!(0, pairs);
}

#[test]
fn example_df_gives_example_board() {
    let file = include_str!("input-test.txt");
    let expected_board = r"..G
._.
#..
";
    assert_eq!(expected_board, Board::new(file, 2).visualise());
}
