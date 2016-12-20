fn checksum(disk_size:usize) -> String {
    let seed = b"00101000101111010";
    let mut data = vec![];
    for b in seed.iter() {
        data.push(if *b == 49 { 1 } else { 0 });
    }

    while data.len() < disk_size {
        let mut copy:Vec<u8> = data.clone();
        copy.reverse();
        copy = copy.into_iter().map(|b| if b == 0 { 1 } else { 0 }).collect();
        data.push(0);
        data.append(&mut copy);
    }
    data.truncate(disk_size);

//    println!("data (len {}): {:?}", data.len(), data);

    let mut checksum_data = data;
    let mut checksum:Vec<u8> = vec![];
    loop {
        for i in 0..checksum_data.len() / 2 {
            checksum.push(if checksum_data[i * 2] == checksum_data[i * 2 + 1] { 1 } else { 0 });
        }

        if checksum.len() % 2 == 1 {
            return checksum.into_iter().map(|b| if b == 0 { "0" } else { "1" }).collect();
        }

        checksum_data = checksum;
        checksum = vec![];
    }
}

fn main() {
    println!("Part 1 checksum: {}", checksum(272));
    println!("Part 2 checksum: {}", checksum(35651584));
}
