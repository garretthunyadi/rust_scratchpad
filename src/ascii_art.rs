/*

 ONLY A STUB RIGHT NOW

*/

extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn main() -> std::io::Result<()> {
    println!("{}", generate_ascii_art(&thread_rng(), 80, 10));
    Ok(())
}

struct AsciiArt {
    width: usize, // in chars
    height: usize,
    art: String,
}

impl AsciiArt {
    fn new(width: usize, height: usize) -> AsciiArt {
        AsciiArt {
            width,
            height,
            art: generate_ascii_art(&thread_rng(), width, height),
        }
    }
}

#[test]
fn ascii_art() {}

//////////// Base functionality /////////////

fn random_string<R: Rng>(rng: &R, len: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(len).collect()
}

fn generate_ascii_art<R: Rng>(rng: &R, width: usize, height: usize) -> String {
    let mut strings: Vec<String> = vec![];
    for _ in 0..height {
        strings.push(random_string(rng, width))
    }
    strings.join("\n")
}

#[test]
fn test_generate_ascii_art() {
    let art = generate_ascii_art(&thread_rng(), 10, 10);
    println!("{}", art);
    assert_eq!(art.split("\n").count(), 10);
    for line in art.split("\n") {
        println!("<<<{}>>>", line);
        assert_eq!(line.len(), 10);
    }
}
