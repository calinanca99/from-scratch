use std::{
    io::{self, Write},
    process::Command,
};

struct Program {
    exec_path: String,
    args: Vec<String>,
}

impl Program {
    fn new(s: String) -> Self {
        let v = s
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if v.is_empty() {
            panic!("Command cannot contain only whitespace");
        }

        let args = if v.len() == 1 {
            vec![]
        } else {
            v[1..].to_vec()
        };

        Self {
            exec_path: v[0].to_string(),
            args,
        }
    }
}

fn main() {
    println!("\n####### shell-rs #######\n");

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        let br = io::stdin().read_line(&mut buf).expect("Cannot read stdin");

        if br == 0 {
            break;
        }

        let program = Program::new(buf);

        let res = match Command::new(program.exec_path).args(program.args).output() {
            Ok(output) => match String::from_utf8(output.stdout) {
                Ok(s) => s,
                Err(e) => e.to_string(),
            },
            Err(e) => e.to_string(),
        };

        println!("{}", res);
    }
}
