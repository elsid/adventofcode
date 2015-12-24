#![feature(str_char)]

fn main() {
    use std::io::{stdin, BufRead};
    let stream = stdin();
    let count = stream.lock().lines()
        .map(|v| v.unwrap())
        .filter(|v| has_pairs_duplicates_without_overlap(v) && has_repeat_with_one_letter_between(v))
        .count();
    println!("{}", count);
}

fn has_pairs_duplicates_without_overlap(line: &str) -> bool {
    use std::collections::HashMap;
    let mut pairs: HashMap<&str, u64> = HashMap::new();
    let mut previous = None;
    for i in 1..line.len() {
        let pair = &line[(i - 1)..(i + 1)];
        if previous == Some(pair) {
            previous = None;
            continue;
        }
        previous = Some(pair);
        if pairs.contains_key(pair) {
            let count = pairs.get_mut(pair).unwrap();
            *count += 1;
        } else {
            pairs.insert(pair, 1);
        }
    }
    pairs.iter().filter(|&(_, &v)| v >= 2).next().is_some()
}

fn has_repeat_with_one_letter_between(line: &str) -> bool {
    (2..line.len()).filter(|&v| line.char_at(v - 2) == line.char_at(v)).next().is_some()
}
