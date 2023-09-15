// Requirements:
// - read from file
// - read from stdin if no file is provided
// - count: words, lines, bytes
// - print to stdout
// - save to file

use std::{fs::File, io::Read};

const CAPACITY: usize = 1024;

fn count_bytes(s: &str) -> usize {
    s.len()
}

fn count_lines(s: &str) -> usize {
    s.lines().collect::<Vec<_>>().len()
}

fn count_words(s: &str) -> usize {
    s.split_whitespace().collect::<Vec<_>>().len()
}

fn main() -> std::io::Result<()> {
    let args = std::env::args();

    if args.len() == 1 {
        panic!("Cannot read from stdin. Provide a file path.")
    }

    let path = args.last().unwrap();
    let mut file = File::open(&path)?;

    let mut buffer = String::with_capacity(CAPACITY);

    file.read_to_string(&mut buffer)?;

    let bytes = count_bytes(&buffer);
    let lines = count_lines(&buffer);
    let words = count_words(&buffer);

    println!("Lines: {lines}\nWords: {words}\nBytes: {bytes}\nFile: {path}");

    Ok(())
}
