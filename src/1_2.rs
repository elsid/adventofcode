#![feature(io)]

use std::io::{stdin, Read};

fn main() {
    let mut sum: i64 = 0;
    for (i, v) in stdin().chars().enumerate() {
        sum += match v.unwrap() {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if sum == -1 {
            println!("{}", i + 1);
            break;
        }
    };
}
