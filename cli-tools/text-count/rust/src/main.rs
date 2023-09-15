use std::{
    fs::File,
    io::{self, Read, Write},
};

use text_count::{cli::Cli, count_bytes, count_lines, count_words};

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let cli = Cli::new(&args).expect("Cannot parse arguments");

    let mut buffer = String::new();

    if let Some(file_path) = cli.file_path() {
        let mut file = File::open(file_path)?;
        file.read_to_string(&mut buffer)?;
    } else {
        // Read from `stdin` until EOF
        loop {
            let bytes_read = io::stdin().read_line(&mut buffer)?;

            if bytes_read == 0 {
                break;
            } else {
                continue;
            }
        }
    }

    let bytes = count_bytes(&buffer);
    let lines = count_lines(&buffer);
    let words = count_words(&buffer);

    let mut res = format!("Lines: {lines}\nWords: {words}\nBytes: {bytes}\n");
    if cli.file_path().is_some() {
        let file_path = format!("File: {}\n", cli.file_path().unwrap());
        res.push_str(&file_path);
    }

    if let Some(output) = cli.output() {
        let mut output_file = File::create(output)?;
        output_file.write_all(res.as_bytes())?;
    } else {
        println!("{res}");
    }

    Ok(())
}
