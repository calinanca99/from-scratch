use std::{fs::File, io::Read};

pub fn open_file(p: &str) -> String {
    let mut buf = String::new();

    File::open(p)
        .expect("Cannot open file")
        .read_to_string(&mut buf)
        .expect("Cannot read file");

    buf
}
