extern crate permutohedron;
use permutohedron::Heap;

use std::collections::VecDeque;
use std::collections::{HashSet, HashMap};

const WIDTH:usize = 181;
const HEIGHT:usize = 39;
const NUM_POINTS:usize = 8;

struct FloorPlan {
    data: [[bool; WIDTH]; HEIGHT],
    points_to_visit: [(usize, usize); NUM_POINTS],
}

impl FloorPlan {
    fn new(file: &str) -> FloorPlan {
        let mut plan = [[true; WIDTH]; HEIGHT];
        let mut points_to_visit = [(0,0); 8];

        for (row, line) in file.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        plan[row][col] = false;
                    },
                    '0'...'7' => {
                        points_to_visit[char.to_digit(10).unwrap() as usize] = (row, col);
                    },
                    '.' => {},
                    _ => unreachable!()
                }
            }
        };

        FloorPlan {
            data: plan,
            points_to_visit: points_to_visit,
        }
    }

    fn neighbours(&self, &(row, col): &(usize, usize)) -> Vec<(usize, usize)> {
        let deltas = [(-1i32, 0), (0, 1), (1, 0), (0, -1)];

        deltas.into_iter()
            .map(|&(dy, dx)| (row as i32 + dy, col as i32 + dx))
            .filter(|&(y, x)| x > 0 && x < WIDTH as i32 && y > 0 && y < HEIGHT as i32 && self.data[y as usize][x as usize])
            .map(|(y, x)| (y as usize, x as usize))
            .collect()
    }

    fn shortest_path_length(&self, from:(usize, usize), to:(usize, usize)) -> Result<usize, ()> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        visited.insert(from.clone());
        queue.push_back(SearchState::new(from, 0));

        while let Some(state) = queue.pop_front() {
            let neighbours = self.neighbours(&state.current);
            for neighbour in neighbours {
                if visited.contains(&neighbour) {
                    continue;
                }
                if neighbour == to {
                    return Ok(state.length_so_far + 1);
                }

                visited.insert(neighbour);
                let new_state = SearchState::new(neighbour, state.length_so_far + 1);
                queue.push_back(new_state);
            }
        }
        Err(())
    }
}

struct SearchState {
    current: (usize, usize),
    length_so_far: usize,
}

impl SearchState {
    fn new(current:(usize, usize), length: usize) -> SearchState {
        SearchState{
            current: current,
            length_so_far: length
        }
    }
}

fn main() {
    let file = include_str!("input.txt");
    let floor_plan = FloorPlan::new(file);

    let mut shortest_distances = HashMap::new();
    for i in 0..NUM_POINTS {
        for j in (i+1)..NUM_POINTS {
            let shortest_dist = floor_plan.shortest_path_length(
                floor_plan.points_to_visit[i], floor_plan.points_to_visit[j]);
            match shortest_dist {
                Ok(sd) => { shortest_distances.insert((i,j), sd); },
                Err(_) => { panic!("Couldn't find shortest distance between {} and {}", i, j); }
            }
        }
    }

    let mut nums = vec![1,2,3,4,5,6,7];

    let heap:Heap<Vec<usize>, usize> = Heap::new(&mut nums);
    let shortest_route_len:Option<usize> = heap.map(|sub_perm| {
        let mut full_perm = sub_perm.clone();
        full_perm.insert(0, 0);
        full_perm.push(0); // Comment out for part 1
        full_perm.windows(2).map(|pair| {
            let a = std::cmp::min(pair[0], pair[1]);
            let b = std::cmp::max(pair[0], pair[1]);
            shortest_distances.get(&(a, b)).unwrap()
        }).sum()
    }).min();

    println!("{:?}", shortest_route_len);
}