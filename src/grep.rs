use std::fs;

// Given a path spec (glob), find all the files where their content is like (something) and return abstracted types

pub fn main() {
    // let data = fs::read_to_string("/etc/hosts").expect("Unable to read file");
    // println!("{}", data);

    // let strings = vec!["tofu", "93", "18"];
    // let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    // println!("Results: {:?}", numbers);

    // let strings = vec!["tofu", "93", "18"];
    // let numbers: Vec<_> = strings
    //     .into_iter()
    //     .map(|s| s.parse::<i32>())
    //     .filter_map(Result::ok)
    //     .collect();
    // println!("Results: {:?}", numbers);

    println!("grep");
    // let paths = fs::read_dir(".").unwrap();
    // for path in paths {
    //     println!("{:?}", path);
    // }

    let paths = fs::read_dir(".")
        .unwrap()
        .map(Result::unwrap)
        .map(|dir_entry| dir_entry.path())
        .collect::<Vec<_>>();

    println!("here");
    //     use glob::glob;
    //     println!();
    //     // "/media/**/*.jpg"
    //     for entry in glob("src/*.rs").expect("Failed to read glob pattern") {
    //         match entry {
    //             Ok(path) => println!("{:?}", path.display()),
    //             Err(e) => println!("{:?}", e),
    //         }
    //     }

    for path in paths {
        print!("x");

        if let Ok(found) = search_in(&path, "foo") {
            if found {
                println!("================= hit: {:?} ======================", path);
            } else {
                print!(".");
            }
        }
    }
    println!("fin");

    //     let r = glob("*").unwrap();
}

// use std::fs::File;
// use std::io::{BufRead, BufReader, Error};
use std::io::{Error, ErrorKind};

fn search_in(path: &std::path::PathBuf, to_match: &str) -> Result<bool, Error> {
    if path.is_dir() {
        print!("<dir>");
        return Err(Error::new(ErrorKind::Other, "Path is directory"));
    }
    print!("a");

    // let input = File::open(path)?;
    // let buffered = BufReader::new(input);

    // let mut cnt = 0;
    // for line in buffered.lines() {
    //     // println!("{}", line?);
    //     cnt += 1;
    // }
    // println!("{}\t{:?}", cnt, path);
    // Ok(false)
    if let Ok(mut reader) = my_reader::BufReader::open(path) {
        println!("b");

        let mut buffer = String::new();

        while let Some(line) = reader.read_line(&mut buffer) {
            println!("c");

            if let Ok(line) = line {
                println!("OK: {}", line.trim());
            } else {
                println!("{:?}", line.err());
            }
        }
    }

    Ok(true)
}

// File Read/Write
// use std::fs::File;
// use std::io::{Write, BufReader, BufRead, Error};

// fn main() -> Result<(), Error> {
//     let path = "lines.txt";

//     let mut output = File::create(path)?;
//     write!(output, "Rust\n💖\nFun")?;

//     let input = File::open(path)?;
//     let buffered = BufReader::new(input);

//     for line in buffered.lines() {
//         println!("{}", line?);
//     }

//     Ok(())
// }

// pub fn files(path: &str) -> Vec<fs::DirEntry> {
// fs::read_dir(".")
//     .unwrap()
//     // .filter_map(|f| f.unwrap())
//     .collect::<Vec<_>>()
// }

// from https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}

#[test]
fn test_my_reader() {
    let mut reader = my_reader::BufReader::open("Cargo.toml").expect("didn't finf the file");
    let mut buffer = String::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        println!("{}", line.expect("unexpected").trim());
    }
}
