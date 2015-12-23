#![feature(iter_arith)]
#![feature(io)]

use std::io::{stdin, Read};

fn main() {
    let sum: i64 = stdin().chars().map(|v| {
        match v.unwrap() {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    }).sum();
    println!("{}", sum);
}
