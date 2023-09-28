use std::io::{self, Write};

fn main() {
    println!("\n####### shell-rs #######\n");

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        let br = io::stdin().read_line(&mut buf).unwrap();

        println!("{buf}");

        if br == 0 {
            break;
        }
    }
}
