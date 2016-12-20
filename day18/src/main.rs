struct Floor {
    tiles: Vec<Vec<bool>>,
    safe_count: usize
}

impl Floor {
    fn new(first_row: &str) -> Floor {
        let mut row = vec![];
        let mut safe_count = 0;
        for c in first_row.chars() {
            match c {
                '.' => { row.push(false); safe_count += 1; },
                '^' => { row.push(true); },
                _ => unreachable!()
            }
        }

        Floor {
            tiles: vec![row],
            safe_count: safe_count
        }
    }

    fn gen_next_row(&mut self) {
        let mut row = vec![];
        {
            let prev_row = &self.tiles[self.tiles.len() - 1];

            for i in 0..prev_row.len() {
                let left = if i > 0 { prev_row[i - 1] } else { false };
                let centre = prev_row[i];
                let right = if i < prev_row.len() - 1 { prev_row[i + 1] } else { false };

                let is_trap = (left && centre && !right) ||
                    (!left && centre && right) ||
                    (left && !centre && !right) ||
                    (!left && !centre && right);

                row.push(is_trap);
                if !is_trap {
                    self.safe_count += 1;
                }
            }
        }

        self.tiles.push(row);
    }

    fn gen_rows(&mut self, num: usize) {
        for _ in 0..num {
            self.gen_next_row();
        }
    }

    fn as_string(&self) -> String {
        self.tiles.iter().map(|row| {
            row.iter()
                .map(|t| if *t { '^' } else { '.' })
                .collect::<String>()
        })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn num_safe_tiles(&self) -> usize {
        // self.tiles.iter().flat_map(|row| row.iter().filter(|tile| !**tile)).count()
        self.safe_count
    }
}

fn main() {
    let mut floor = Floor::new(".^^^^^.^^.^^^.^...^..^^.^.^..^^^^^^^^^^..^...^^.^..^^^^..^^^^...^.^.^^^^^^^^....^..^^^^^^.^^^.^^^.^^");
    floor.gen_rows(39);
    println!("Part 1: {}", floor.num_safe_tiles());

    floor.gen_rows(400000 - 40);
    println!("Part 2: {}", floor.num_safe_tiles());
}

#[test]
fn three_line_example() {
    let mut floor = Floor::new("..^^.");
    floor.gen_rows(2);
    assert_eq!(floor.as_string(), r"..^^.
.^^^^
^^..^");
}

#[test]
fn ten_line_example() {
    // ..^^.
    let mut floor = Floor::new(".^^.^.^^^^");
    floor.gen_rows(9);
    assert_eq!(floor.as_string(), r".^^.^.^^^^
^^^...^..^
^.^^.^.^^.
..^^...^^^
.^^^^.^^.^
^^..^.^^..
^^^^..^^^.
^..^^^^.^^
.^^^..^.^^
^^.^^^..^^");
    assert_eq!(floor.num_safe_tiles(), 38);
}
