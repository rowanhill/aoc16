use std::collections::HashMap;
use std::collections::VecDeque;

const FAV_NUM:usize = 1358;

struct Node {
    x: usize,
    y: usize,
    is_wall: bool,
    steps: usize,
    prev: Option<(usize, usize)>
}

impl Node {
    fn new(x: usize, y: usize) -> Node {
        Node {
            x: x,
            y: y,
            is_wall: Self::is_wall(x, y),
            steps: std::usize::MAX,
            prev: None
        }
    }

    fn is_wall(x: usize, y: usize) -> bool {
        let mut num = (x*x + 3*x + 2*x*y + y + y*y) + FAV_NUM;
        let mut bit_count = 0;
        while num > 0 {
            if num % 2 == 1 {
                bit_count += 1;
            }
            num = num / 2;
        }
        bit_count % 2 == 1
    }

    fn neighbour_coords(&self) -> Vec<(usize, usize)> {
        let deltas = [(-1i32, 0), (0, 1), (1, 0), (0, -1)];

        let mut results = vec![];
        for delta in &deltas {
            let new_x = self.x as i32 + delta.0;
            let new_y = self.y as i32 + delta.1;
            if new_x >= 0 && new_y >= 0 {
                results.push((new_x as usize, new_y as usize));
            }
        }
        results
    }

    fn update(&mut self, other_coords:(usize, usize), other_steps: usize) -> bool {
        if self.is_wall || self.steps <= other_steps + 1 {
            return false;
        }

        self.steps = other_steps + 1;
        self.prev = Some(other_coords);

        true
    }
}

struct SearchState {
    target: (usize, usize),
    max_steps: usize,
    found_target: bool,
    current_coords: (usize, usize),
    current_steps: usize
}

struct Searcher {
    nodes: HashMap<(usize, usize), Node>,
    queue: VecDeque<(usize, usize)>
}

impl Searcher {
    fn new(mut start_node: Node) -> Searcher {
        let mut searcher = Searcher {
            nodes: HashMap::new(),
            queue: VecDeque::new()
        };

        start_node.steps = 0;
        searcher.add(start_node);

        searcher
    }

    fn add(&mut self, node: Node) {
        self.queue.push_back((node.x, node.y));
        self.nodes.insert((node.x, node.y), node);
    }

    fn pop(&mut self) -> Option<((usize, usize), usize)> {
        self.queue.pop_front()
            .and_then(|coord| self.nodes.get(&coord))
            .map(|node| ((node.x, node.y), node.steps))
    }

    fn get_or_create(&mut self, (x, y):(usize, usize)) -> &mut Node {
        self.nodes.entry((x, y)).or_insert(Node::new(x, y))
    }

    fn search(&mut self, target:(usize, usize), max_steps: usize) {
        loop {
            let (coord, current_steps) = match self.pop() {
                Some(t) => t,
                None => break
            };

            let mut state = SearchState {
                target: target,
                max_steps: max_steps,
                found_target: false,
                current_coords: coord,
                current_steps: current_steps
            };

            for neighbour_coord in self.get_neighbour_coords_to_process(&mut state) {
                self.queue.push_back(neighbour_coord);
            }
        }

        let count = self.nodes.iter().filter(|&(_, node)| node.steps <= max_steps).count();
        println!("{} nodes reachable in <= {} steps", count, max_steps);
    }

    fn get_neighbour_coords_to_process(&mut self, state: &mut SearchState) -> Vec<(usize, usize)> {
        let neighbour_coords = {
            let node = self.get_or_create(state.current_coords);
            node.neighbour_coords()
        };

        neighbour_coords.into_iter()
            .filter(|& neighbour_coord| {
                let mut neighbour_node = self.get_or_create(neighbour_coord);

                if neighbour_node.update(state.current_coords, state.current_steps) {
                    if state.target == neighbour_coord {
                        println!("Reached {:?} in {} steps", state.target, neighbour_node.steps);
                        state.found_target = true;
                    }

                    if neighbour_node.steps < state.max_steps || !state.found_target {
                        return true;
                    }
                }

                return false;
            })
            .collect()
    }
}

fn main() {
    let mut searcher = Searcher::new(Node::new(1, 1));
    searcher.search((31, 39), 50);
}
