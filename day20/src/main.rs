fn part1(input: &mut Vec<(usize, usize)>) {
    let mut lowest = 0;
    for &mut (from, to) in input {
        if from <= lowest {
            lowest = std::cmp::max(lowest, to + 1);
        } else {
            break;
        }
    }

    println!("{}", lowest);
}

fn get_available_ranges(input: &mut Vec<(usize, usize)>, max: usize) -> Vec<(usize, usize)> {
    let mut available = vec![];

    let mut lowest = 0;
    for &mut (from, to) in input {
        if from <= lowest {
            lowest = std::cmp::max(lowest, to + 1);
        } else {
            available.push((lowest, from - 1));
            lowest = to + 1;
        }
    }

    available.push((lowest, max));

    available
}

fn main() {
    let mut input:Vec<(usize, usize)> = include_str!("input.txt").lines()
        .map(|l| {
            let nums:Vec<usize> = l.split("-")
                .map(|p| p.parse::<usize>().unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .collect();
    input.sort_by(|&(a, _), &(c, _)| a.cmp(&c));

    let available = get_available_ranges(&mut input, 4294967295);
    println!("Lowest available IP: {}", available[0].0);

    let sum:usize = available.into_iter().map(|(from, to)| to + 1 - from).sum();
    println!("Sum of available ranges: {}", sum);

}
