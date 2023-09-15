pub struct Cli {
    file_path: Option<String>,
    output: Option<String>,
}

impl Cli {
    pub fn new(args: &[String]) -> Result<Self, String> {
        let mut cli = Cli {
            file_path: None,
            output: None,
        };

        if let Some(i) = Self::get_flag_index(args, "--file", "-f") {
            match args.get(i + 1) {
                Some(file_path) => cli.file_path = Some(file_path.to_string()),
                None => return Err("Usage: `--file <file_path>`".to_string()),
            }
        }

        if let Some(i) = Self::get_flag_index(args, "--output", "-o") {
            match args.get(i + 1) {
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

    fn get_flag_index(args: &[String], long_pattern: &str, short_pattern: &str) -> Option<usize> {
        args.iter()
            .position(|arg| arg == long_pattern || arg == short_pattern)
    }
}
