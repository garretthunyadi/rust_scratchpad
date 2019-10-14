// time insites list -n 50000 all | cargo run | sort | uniq -c | sort
use crc::crc32;
use std::io::{self, Read};

pub fn main() -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let hash_pairs: Vec<_> = buffer
        .lines()
        .map(|line| {
            let cs = crc32::checksum_ieee(line.as_bytes());
            // (line, cs, cs % 10)
            (line, cs, cs % 10)
        })
        .collect();

    for pair in hash_pairs {
        // println!("{},{}", pair.1, pair.0);
        println!("{}", pair.2);
    }

    Ok(())
}
