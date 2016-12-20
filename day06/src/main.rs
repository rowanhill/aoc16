use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let input_width = 8;

    let mut maps:Vec<HashMap<char, usize>> = vec![];
    for _ in 0..input_width {
        maps.push(HashMap::new());
    }

    for line in input.lines() {
        for (index, char) in line.chars().enumerate() {
            let ref mut map:HashMap<char, usize> = maps[index];
            let count = map.entry(char).or_insert(0);
            *count += 1;
        }
    }

    let part1:String = maps.iter().map(|map| {
        map.into_iter().max_by_key(|&(_, count)| count).unwrap().0
    }).map(|&c| c).collect();

    let part2:String = maps.iter().map(|map| {
        map.into_iter().min_by_key(|&(_, count)| count).unwrap().0
    }).map(|&c| c).collect();

    println!("{}", part1);
    println!("{}", part2);
}
