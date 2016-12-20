extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::collections::HashMap;

fn main() {
    let salt = b"jlmsuwbz";
    let mut md5 = Md5::new();

    let mut triple_indexes_by_char:HashMap<u8, Vec<usize>> = HashMap::new();

    let mut key_count = 0;

    'outer: for hash_counter in 0..std::usize::MAX {
        md5.input(salt);
        md5.input(hash_counter.to_string().as_bytes());

        let mut result = [0; 16];
        md5.result(&mut result);

        let mut nibbles = [0; 32];
        for (idx, byte) in result.iter().enumerate() {
            nibbles[idx*2] = (byte & (15 << 4)) >> 4;
            nibbles[idx*2 + 1] = byte & 15;
        }

        for idx in 0..30 {
            let a = nibbles[idx + 0];
            let b = nibbles[idx + 1];
            let c = nibbles[idx + 2];
            if (a == b) && (b == c) {
                //                if hash_counter >= 22728 {
                //                    println!("Found triple of {} at {}", a, hash_counter);
                //                }
                let mut vec = triple_indexes_by_char.entry(a).or_insert(vec![]);
                vec.push(hash_counter);
                break; // only consider first triple in a hash
            }
        }
        // TODO: Only look for quintuple if found a triple
        for idx in 0..28 {
            let a = nibbles[idx + 0];
            let b = nibbles[idx + 1];
            let c = nibbles[idx + 2];
            let d = nibbles[idx + 3];
            let e = nibbles[idx + 4];

            if a == b && b == c && c == d && d == e {
                //                if hash_counter >= 22728 {
                //                    println!("Found quituple of {} at {}", a, hash_counter);
                //                }
                {
                    let triple_indexes = triple_indexes_by_char.get(&a).unwrap();
                    for triple_index in triple_indexes {
                        if hash_counter < *triple_index + 1000 && *triple_index < hash_counter {
                            key_count += 1;
                            println!("Found key {} at {}, checking {}: {:?}", key_count, triple_index, hash_counter, nibbles);

                            if key_count >= 64 {
                                break 'outer;
                            }
                        }
                    }
                }
                triple_indexes_by_char.insert(a, vec![hash_counter]);
            }
        }

        md5.reset();
    }
}
