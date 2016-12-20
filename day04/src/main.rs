use std::collections::HashMap;
use std::ops::Index;

static ALPHA: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let input = include_str!("input.txt");

    println!("Sum of valid ids: {}", sum_valid_ids(input));

    let shift_test = ParsedName {
        letter_parts: vec!["qzmt", "zixmtkozy", "ivhz"],
        id: 343,
        checksum: ""
    };
    println!("Test decrypting: {}", shift_test.decrypt());

    for line in input.lines() {
        let name = ParsedName::new(line);
        if name.is_valid() {
            let decrypted = name.decrypt();
            if decrypted.starts_with("north") {
                println!("{}: {}", name.id, name.decrypt());
            }
        }
    }
}

#[derive(Debug)]
struct CharCount<'a> {
    char: &'a char,
    count: &'a u32
}

#[derive(Debug)]
struct ParsedName<'a> {
    letter_parts: Vec<&'a str>,
    id: u32,
    checksum: &'a str
}

impl<'a> ParsedName<'a> {
    fn new(name: &str) -> ParsedName {
        let parts = name.split("-");
        let (letter_parts, number_parts):(Vec<&str>, Vec<&str>) = parts.partition(
            |p| p.starts_with(|c:char| c.is_alphabetic())
        );
        let number_part = number_parts[0];

        let number_part_splits:Vec<&str> = number_part.split("[").collect();
        let id_str = number_part_splits[0];
        let checksum_splits:Vec<&str> = number_part_splits[1].split("]").collect();
        let checksum:&str = checksum_splits[0];

        ParsedName {
            letter_parts: letter_parts,
            id: id_str.parse::<u32>().unwrap(),
            checksum: checksum
        }
    }

    fn is_valid(&self) -> bool {
        let mut counts = HashMap::new();
        for letter_part in &self.letter_parts {
            for char in letter_part.chars() {
                let counter = counts.entry(char).or_insert(0);
                *counter += 1;
            }
        }

        let mut char_counts:Vec<CharCount> = counts.iter()
            .map(|(k, v)| CharCount{ char: k, count: v })
            .collect();
        char_counts.sort_by(|a, b| {
            let count_ord = b.count.cmp(a.count);
            if count_ord == std::cmp::Ordering::Equal {
                a.char.cmp(b.char)
            } else {
                count_ord
            }
        });

        let expected_checksum:String = char_counts.index(0..5).iter()
            .map(|cc| cc.char.clone()) // convert &char to char
            .collect(); // concatenate to String

        self.checksum.eq(&expected_checksum)
    }

    fn decrypt(&self) -> String {
        (&self.letter_parts).into_iter().map(|letter_part| {
            letter_part.chars().map(|c| {
                let pos = ALPHA.chars().position(|a| c == a).unwrap();
                ALPHA.chars().nth((pos + self.id as usize) % 26).unwrap().clone()
            }).collect::<String>()
        }).collect::<Vec<String>>()
            .join(" ")
    }
}

fn sum_valid_ids(input: &str) -> u32 {
    input.lines().map(|name| {
        let parsed_name = ParsedName::new(name);

        if parsed_name.is_valid() {
            Some(parsed_name.id)
        } else {
            None
        }
    }).fold(0, |acc, id| {
        if let Some(id_inner) = id {
            acc + id_inner
        } else {
            acc
        }
    })
}

#[test]
fn sum_of_valid_ids_from_test_input_is_1514() {
    let test_input = r"aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    assert_eq!(sum_valid_ids(test_input), 1514);
}