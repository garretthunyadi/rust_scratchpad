// use std::fs::File;
// use std::io::{BufRead, BufReader, Error};
use glob::{glob, Paths, PatternError};
// use std::fs;
use regex::Regex;
use std::io::{Error, ErrorKind};

// Given a path spec (glob), find all the files where their content is like (something) and return abstracted types

// [x] give the search function a fn to call for each result, rather that having it collect all of the
//    results
// [] command-line
// [] metrics: file and line counts
// [x] glob
// [] regex
// [] multi-threaded
// [] perf monitoring

enum Query {
    String(String),
    Regex(Regex),
}

pub fn main() {
    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let re = Regex::new(r"Ga..ett").unwrap();
    // assert!(re.is_match("2014-01-01"));

    // Garrett
    // Garnett
    println!("grep");
    // "/media/**/*.jpg"
    // match search(&Query::String(String::from("fn")), glob("src/*.rs"), &{
    match search(&Query::Regex(re), glob("src/*.rs"), &{
        |hit: &str| print!("{}", hit)
    }) {
        Ok((num_hits, num_files, num_lines)) => {
            println!(
                "\nfin: {} hits in {} files with {} total lines.",
                num_hits, num_files, num_lines
            );
        }
        Err(err) => println!("err: {}", err),
    }
}

fn search(
    query: &Query,
    paths: Result<Paths, PatternError>,
    on_hit: &dyn Fn(&str),
) -> Result<(usize, usize, usize), Error> {
    let mut num_hits = 0;
    let mut num_lines = 0;
    let mut num_files = 0;
    for entry in paths.expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                num_files += 1;
                if let Ok((hits, lines_searched)) = search_in(&path, query) {
                    num_hits += hits.len();
                    num_lines += lines_searched;
                    for hit in hits {
                        on_hit(&hit);
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    Ok((num_hits, num_files, num_lines))
}

#[test]
fn test_search() {
    let _ = search(
        &Query::String(String::from("something")),
        glob("src/*.rs"),
        &{ |hit: &str| println!("HIT : {}", hit) },
    );
}

fn search_in(path: &std::path::PathBuf, query: &Query) -> Result<(Vec<String>, usize), Error> {
    let mut hits = vec![];
    let mut num_lines = 0;

    if path.is_dir() {
        return Err(Error::new(ErrorKind::Other, "Path is directory"));
    }

    if let Ok(mut reader) = my_reader::BufReader::open(path) {
        let mut buffer = String::new();

        while let Some(line) = reader.read_line(&mut buffer) {
            if let Ok(line) = line {
                num_lines += 1;
                match query {
                    Query::String(term) => {
                        if line.contains(term) {
                            hits.push(line.clone());
                        }
                    }
                    Query::Regex(regex) => {
                        if regex.is_match(line) {
                            hits.push(line.clone());
                        }
                    }
                }
            } else {
                // println!("{:?}", line.err());
            }
        }
    }

    Ok((hits, num_lines))
}

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
