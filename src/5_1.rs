extern crate regex;
extern crate itertools;

use std::io::{stdin, BufRead, Read};
use regex::Regex;
use itertools::Itertools;

fn main() {
    let stream = stdin();
    let invalid = Regex::new(r"ab|cd|pq|xy").unwrap();
    let vowels = Regex::new(r"(.*[aeiou].*){3}").unwrap();
    let count = stream.lock().lines()
        .map(|v| v.unwrap())
        .filter(|v| !invalid.is_match(v) && vowels.is_match(v) && has_duplicates(v))
        .count();
    println!("{}", count);
}

fn has_duplicates(line: &str) -> bool {
    line.chars()
        .group_by(|&v| v)
        .map(|(_, group)| group.len())
        .filter(|&v| v >= 2)
        .next().is_some()
}
