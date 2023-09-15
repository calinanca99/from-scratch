use std::{
    fs::File,
    io::{self, Read, Write},
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
    output: Option<String>,
}

impl Cli {
    pub fn new(args: &[String]) -> Result<Self, String> {
        let mut cli = Cli {
            file_path: None,
            output: None,
        };

        if let Some(i) = args.iter().position(|arg| arg == "--file") {
            match args.iter().nth(i + 1) {
                Some(file_path) => cli.file_path = Some(file_path.to_string()),
                None => return Err("Usage: `--file <file_path>`".to_string()),
            }
        }

        if let Some(i) = args.iter().position(|arg| arg == "--output") {
            match args.iter().nth(i + 1) {
                Some(output) => cli.output = Some(output.to_string()),
                None => return Err("Usage: `--output <output>`".to_string()),
            }
        }

        Ok(cli)
    }

    pub fn file_path(&self) -> Option<String> {
        self.file_path.clone()
    }

    pub fn output(&self) -> Option<String> {
        self.output.clone()
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

            if read == 0 {
                break;
            } else {
                continue;
            }
        }
    }

    let bytes = count_bytes(&buffer);
    let lines = count_lines(&buffer);
    let words = count_words(&buffer);

    let res = format!("Lines: {lines}\nWords: {words}\nBytes: {bytes}\n");

    if let Some(output) = cli.output() {
        let mut output_file = File::create(output)?;
        output_file.write_all(res.as_bytes())?;
    } else {
        println!("{res}");
    }

    Ok(())
}
