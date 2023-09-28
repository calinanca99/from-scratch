use std::{
    io::{self, Write},
    process::Command,
};

fn main() {
    println!("\n####### shell-rs #######\n");

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        let br = io::stdin().read_line(&mut buf).expect("Cannot read stdin");

        // TODO: Handle if input is just whitespace
        if br == 0 {
            break;
        }

        // Don't read the trailing '\n' from `buf`
        let cmd = &buf[..br - 1];
        let res = match Command::new(cmd).output() {
            Ok(output) => match String::from_utf8(output.stdout) {
                Ok(s) => s,
                Err(e) => e.to_string(),
            },
            Err(e) => e.to_string(),
        };

        println!("{}", res);
    }
}
