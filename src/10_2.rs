#![feature(io)]

use std::io::{stdin, Read};

fn main() {
    let mut sequence: Vec<u8> = stdin().chars()
        .map(|v| match v.unwrap() {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            _ => 0,
        })
        .filter(|v| *v != 0)
        .collect::<_>();
    for _ in 0..50 {
        sequence = transform(sequence);
    }
    println!("{}", sequence.len());
}

fn transform(sequence: Vec<u8>) -> Vec<u8> {
    let mut previous = None;
    let mut count: u8 = 0;
    let mut result = vec![];
    for v in sequence {
        if Some(v) == previous {
            count += 1;
        } else {
            if previous.is_some() {
                result.push(count);
                result.push(previous.unwrap());
            }
            previous = Some(v);
            count = 1;
        }
    }
    if previous.is_some() {
        result.push(count);
        result.push(previous.unwrap());
    }
    result
}
