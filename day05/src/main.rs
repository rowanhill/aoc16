extern crate crypto;

use crypto::md5::*;
use crypto::digest::Digest;

fn main() {
    let mut md5 = Md5::new();
    md5.input_str("reyedfim");

    let mut answer = String::new();
    let mut answer2 = ['-'; 8];
    let mut count = 0;
    let mut answer2_count = 0;

    while answer.len() < 8 || answer2_count < 8 {
        md5.input_str(&count.to_string());
        let result = md5.result_str();

        if result.starts_with("00000") {
            let sixth = result.chars().nth(5).unwrap();

            if answer.len() < 8 {
                answer.push(sixth);
                println!("NEW DIGIT: {}", answer);
            }

            match sixth.to_digit(10) {
                Some(sixth_digit) => {
                    if sixth_digit < 8 && answer2[sixth_digit as usize] == '-' {
                        let seventh = result.chars().nth(6).unwrap();
                        answer2[sixth_digit as usize] = seventh;
                        answer2_count += 1;
                        println!("NEW DIGIT 2: {:?}", answer2);
                    }
                },
                None => {}
            }
        }

        if count % 50000 == 0 {
            println!("{}", count);
        }

        md5.reset();
        count += 1;
    }

    println!("Answer: {}", answer);
    println!("Answer2: {:?}", answer2);
}
