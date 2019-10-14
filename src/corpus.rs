use std::env;
use std::io::{self, Read};
use std::io::{Error, ErrorKind};

pub fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (op, other) = parse_args(&args);
    match op.as_ref() {
        "wc" => {
            if let Ok(wc) = word_count_from_stdin() {
                println!("{:?}", wc);
            }
            Ok(())
        }
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            "op must be \"wc\" (currently)",
        )),
    }
}

fn word_count_from_stdin() -> io::Result<usize> {
    println!("WC/todo");
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.split_whitespace().count())
}

// supports two levels
fn parse_args(args: &[String]) -> (String, Option<&String>) {
    match args.get(1) {
        Some(op) => (op.to_string(), args.get(2)),
        None => ("TheDefaultVal".to_string(), None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        assert_eq!(
            parse_args(&["exec_name".to_string(), "aaa".to_string()]),
            ("aaa".to_string(), None)
        );
        assert_eq!(
            parse_args(&[
                "exec_name".to_string(),
                "aaa".to_string(),
                "bbb".to_string()
            ]),
            ("aaa".to_string(), Some(&"bbb".to_string()))
        );
    }
}
