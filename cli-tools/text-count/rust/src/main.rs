// Requirements:
// - read from file
// - read from stdin if no file is provided
// - count: words, lines, bytes
// - print to stdout
// - save to file

use std::{
    fs::File,
    io::{self, Read},
};

fn count_bytes(s: &str) -> usize {
    s.len()
}

fn count_lines(s: &str) -> usize {
    s.lines().count()
}

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

struct Cli {
    file_path: Option<String>,
}

impl Cli {
    pub fn new(args: &[String]) -> Result<Self, String> {
        if let Some(i) = args.iter().position(|arg| arg == "--file") {
            match args.iter().nth(i + 1) {
                Some(file_path) => Ok(Cli {
                    file_path: Some(file_path.to_string()),
                }),
                None => Err("Usage: `--file <file_path>`".to_string()),
            }
        } else {
            Ok(Cli { file_path: None })
        }
    }

    pub fn file_path(self) -> Option<String> {
        self.file_path
    }
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let cli = Cli::new(&args).expect("Cannot parse arguments");

    let mut buffer = String::new();

    if let Some(file_path) = cli.file_path() {
        let mut file = File::open(&file_path)?;
        file.read_to_string(&mut buffer)?;
    } else {
        loop {
            let input = io::stdin();
            let read = input.read_line(&mut buffer)?;

            if read == 1 {
                buffer = buffer[..buffer.len() - 1].to_string();
                break;
            } else {
                continue;
            }
        }
    }

    let bytes = count_bytes(&buffer);
    let lines = count_lines(&buffer);
    let words = count_words(&buffer);

    println!("Lines: {lines}\nWords: {words}\nBytes: {bytes}\n");

    Ok(())
}
