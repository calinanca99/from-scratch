pub mod cli;

/// Returns the number of bytes.
pub fn count_bytes(s: &str) -> usize {
    s.len()
}

/// Returns the number of lines.
pub fn count_lines(s: &str) -> usize {
    s.lines().count()
}

/// Returns the number of words.
pub fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}
