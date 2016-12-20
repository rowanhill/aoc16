use std::collections::VecDeque;
use std::collections::HashSet;

const NUM_PAIRS:usize = 7; // 5 for part 1
const NUM_FLOORS:usize = 4;
const NUM_HASH_PARTS:usize = NUM_FLOORS*NUM_FLOORS + 1;

#[derive(Debug)]
enum ComponentType {
    Generator,
    Microchip
}

#[derive(Debug, Clone, Copy)]
struct Pair {
    element: char,
    gen_floor: usize,
    chip_floor: usize
}

impl Pair {
    fn new(element: char, gf: usize, cf: usize) -> Pair {
        Pair {
            element: element,
            gen_floor: gf,
            chip_floor: cf
        }
    }
}

#[derive(Debug)]
struct Item {
    pair: Pair,
    c_type: ComponentType
}

impl Item {
    fn floor(&self) -> usize {
        match self.c_type {
            ComponentType::Generator => self.pair.gen_floor,
            ComponentType::Microchip => self.pair.chip_floor
        }
    }
}

struct State {
    moves: usize,
    elevator_floor: usize,
    pairs: [Pair; NUM_PAIRS]
}


impl State {
    fn new(elevator_floor: usize, pairs: [Pair; NUM_PAIRS]) -> State {
        State {
            moves: 0,
            elevator_floor: elevator_floor,
            pairs: pairs
        }
    }

    fn print(&self) {
        for i in 0..NUM_FLOORS {
            let floor = NUM_FLOORS - 1 - i;

            print!("F{} ", floor + 1);

            if floor == self.elevator_floor {
                print!("E  ");
            } else {
                print!(".  ");
            }

            for i in 0..NUM_PAIRS {
                let pair = self.pairs[i];
                if floor == pair.gen_floor {
                    print!("{}G ", pair.element)
                } else {
                    print!(".  ");
                }
                if floor == pair.chip_floor {
                    print!("{}M ", pair.element)
                } else {
                    print!(".  ");
                }
            }

            println!("");
        }
    }

    fn hash_counts(&self) -> [u32; NUM_HASH_PARTS] {
        let mut counts = [0u32; NUM_HASH_PARTS];

        counts[0] = self.elevator_floor as u32;
        for i in 0..NUM_PAIRS {
            let pair = self.pairs[i];
            counts[pair.gen_floor*4 + pair.chip_floor + 1] += 1;
        }

        counts
    }

    fn is_end_state(&self) -> bool {
        for i in 0..NUM_PAIRS {
            let pair = self.pairs[i];
            if pair.gen_floor != NUM_FLOORS - 1 || pair.chip_floor != NUM_FLOORS - 1 {
                return false;
            }
        }
        true
    }

    fn move_items(&self, item1: &Item, item2: &Option<Item>, to_floor: usize) -> State {
        if item1.floor() != self.elevator_floor {
            self.print();
            panic!("Expected {:?} to start on floor index {} (to {}) but it isn't", item1, self.elevator_floor, to_floor);
        }
        if let &Some(ref i2) = item2 {
            if i2.floor() != self.elevator_floor {
                self.print();
                panic!("Expected {:?} to start on floor index {} (to {}) but it isn't", i2, self.elevator_floor, to_floor);
            }
        }

        let mut new_pairs = self.pairs.clone();
        for mut pair in &mut new_pairs {
            if pair.element == item1.pair.element {
                match item1.c_type {
                    ComponentType::Microchip => {
                        pair.chip_floor = to_floor
                    },
                    ComponentType::Generator => {
                        pair.gen_floor = to_floor
                    }
                }
            }
            if let &Some(ref i) = item2 {
                if pair.element == i.pair.element {
                    match i.c_type {
                        ComponentType::Microchip => {
                            pair.chip_floor = to_floor
                        },
                        ComponentType::Generator => {
                            pair.gen_floor = to_floor
                        }
                    }
                }
            }
        }

        State {
            moves: self.moves + 1,
            elevator_floor: to_floor,
            pairs: new_pairs
        }
    }

    fn is_floor_safe(&self, floor_index: usize) -> bool {
        let mut lone_gen = false;
        let mut lone_chip = false;

        for i in 0..NUM_PAIRS {
            let pair = self.pairs[i];
            if pair.gen_floor == floor_index && pair.chip_floor != floor_index {
                lone_gen = true;
            } else if pair.chip_floor == floor_index && pair.gen_floor != floor_index {
                lone_chip = true;
            }
        }

        !(lone_gen && lone_chip)
    }

    fn try_and_push(&self, item1: Item, item2opt : Option<Item>, queue: &mut UniqueQueue) -> Option<State> {
        if item1.floor() != self.elevator_floor {
            return None;
        }
        if let Some(ref item2) = item2opt {
            if item2.floor() != self.elevator_floor {
                return None;
            }
        }

        let can_go_up = self.elevator_floor < NUM_FLOORS - 1;
        let can_go_down = self.elevator_floor > 0;

        if can_go_up {
            let new_floor = self.elevator_floor + 1;
            let new_state = self.move_items(&item1, &item2opt, new_floor);
            if new_state.is_end_state() {
                return Some(new_state)
            }
            if new_state.is_floor_safe(new_floor) {
                queue.try_add(new_state);
            }
        }
        if can_go_down {
            let new_floor = self.elevator_floor - 1;
            let new_state = self.move_items(&item1, &item2opt, new_floor);
            if new_state.is_end_state() {
                return Some(new_state)
            }
            if new_state.is_floor_safe(new_floor) {
                queue.try_add(new_state);
            }
        }

        None
    }

    fn push_next_possible_states(&self, mut queue: &mut UniqueQueue) -> Option<State> {
        for i1 in 0..NUM_PAIRS {
            let pair1 = self.pairs[i1];
            if let Some(s) = self.try_and_push(Item { pair: pair1, c_type: ComponentType::Generator }, None, &mut queue) {
                return Some(s)
            }
            if let Some(s) = self.try_and_push(Item { pair: pair1, c_type: ComponentType::Microchip }, None, &mut queue) {
                return Some(s)
            }
            for i2 in i1+1..NUM_PAIRS {
                let pair2 = self.pairs[i2];

                if let Some(s) = self.try_and_push(Item { pair: pair1, c_type: ComponentType::Generator }, Some(Item { pair: pair2, c_type: ComponentType::Generator }), &mut queue) {
                    return Some(s)
                }
                if let Some(s) = self.try_and_push(Item { pair: pair1, c_type: ComponentType::Microchip }, Some(Item { pair: pair2, c_type: ComponentType::Microchip }), &mut queue) {
                    return Some(s)
                }

                if let Some(s) = self.try_and_push(Item { pair: pair1, c_type: ComponentType::Generator }, Some(Item { pair: pair2, c_type: ComponentType::Generator }), &mut queue) {
                    return Some(s)
                }
                if let Some(s) = self.try_and_push(Item { pair: pair1, c_type: ComponentType::Microchip }, Some(Item { pair: pair2, c_type: ComponentType::Microchip }), &mut queue) {
                    return Some(s)
                }
            }
        }

        None
    }
}

struct UniqueQueue {
    queue: VecDeque<State>,
    history: HashSet<[u32;NUM_HASH_PARTS]>
}

impl UniqueQueue {
    fn new() -> UniqueQueue {
        UniqueQueue {
            queue: VecDeque::new(),
            history: HashSet::new()
        }
    }

    fn try_add(&mut self, state: State) {
        if self.history.insert(state.hash_counts()) {
//            println!("New state: {}", state.hash());
//            state.explain_hash();
//            state.print();
//            println!("");
            self.queue.push_back(state);
        }
//            else {
//            println!("Redundant state: {}", state.hash());
//            state.explain_hash();
//            state.print();
//            println!("");
//        }
    }

    fn unshift(&mut self) -> Option<State> {
        self.queue.pop_front()
    }
}

fn main() {
    let mut queue = UniqueQueue::new();

    let init_state = State::new(0, [
        Pair::new('S', 0, 0),
        Pair::new('P', 0, 0),
        Pair::new('T', 1, 2),
        Pair::new('R', 1, 1),
        Pair::new('C', 1, 1),
        Pair::new('E', 0, 0), // Comment out for part 1
        Pair::new('D', 0, 0)  // Comment out for part 1
    ]);

    queue.try_add(init_state);

    let mut max_queue_size = 0;
    while let Some(state) = queue.unshift() {
        if queue.queue.len() + 1 > max_queue_size {
            max_queue_size = queue.queue.len() + 1;
        }
        if let Some(final_state) = state.push_next_possible_states(&mut queue) {
            println!("Found final state in {} moves, having inspected {} states. {} states remain on the queue, which peaked at {}",
                     final_state.moves, queue.history.len(), queue.queue.len(), max_queue_size);
            println!("");
            final_state.print();
            break;
        }
    }
}
