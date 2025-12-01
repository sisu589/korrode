// src/sources.rs
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Iterator over the newline-separated wordlist.
pub fn dict_iter(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Unable to open wordlist");
    BufReader::new(file).lines().filter_map(Result::ok)
}

/// Numeric brute-force generator up to 'max_len' digits (zero-padded)
pub fn brute_force_numeric(max_len: usize) -> impl Iterator<Item = String> {
    let max = 10usize.pow(max_len as u32);
    (0..max).map(move |n| format!("{:0width$}", n, width = max_len))
}
