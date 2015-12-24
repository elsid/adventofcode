#![feature(iter_arith)]

extern crate regex;

use regex::Regex;

struct Point {
    pub x: usize,
    pub y: usize,
}

struct Line {
    pub begin: Point,
    pub end: Point,
}

struct Area {
    pub action: fn(&mut i64) -> (),
    pub line: Line,
}

struct Grid {
    values: Vec<i64>,
}

struct Parser {
    regex: Regex,
}

fn main() {
    use std::io::{stdin, BufRead};
    let input = stdin();
    let parser = Parser::new();
    let mut grid = Grid::new();
    for line in input.lock().lines() {
        let area = parser.parse(&line.unwrap()[..]);
        grid.apply(&area);
    }
    let sum: i64 = grid.values().iter().sum();
    println!("{}", sum);
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            regex: Regex::new(r"(?P<action>toggle|turn on|turn off) (?P<begin_x>\d+),(?P<begin_y>\d+) through (?P<end_x>\d+),(?P<end_y>\d+)").unwrap()
        }
    }

    pub fn parse(&self, line: &str) -> Area {
        let captures = self.regex.captures(line).unwrap();
        let action: fn(&mut i64) -> () = match captures.name("action").unwrap() {
            "toggle" => toggle,
            "turn on" => turn_on,
            "turn off" => turn_off,
            _ => do_nothing,
        };
        let begin_x = captures.name("begin_x").unwrap().parse::<usize>().unwrap();
        let begin_y = captures.name("begin_y").unwrap().parse::<usize>().unwrap();
        let end_x = captures.name("end_x").unwrap().parse::<usize>().unwrap();
        let end_y = captures.name("end_y").unwrap().parse::<usize>().unwrap();
        Area {
            action: action,
            line: Line {
                begin: Point {x: begin_x, y: begin_y},
                end: Point {x: end_x, y: end_y},
            }
        }
    }
}

impl Grid {
    pub fn new() -> Grid {
        Grid {values: (0..1000*1000).map(|_| 0).collect::<_>()}
    }

    pub fn apply(&mut self, area: &Area) {
        for x in area.line.begin.x..area.line.end.x + 1 {
            for y in area.line.begin.y..area.line.end.y + 1 {
                let action = area.action;
                action(&mut self.values[x + y * 1000]);
            }
        }
    }

    pub fn values(&self) -> &Vec<i64> {
        &self.values
    }
}

fn toggle(value: &mut i64) {
    *value += 2;
}

fn turn_on(value: &mut i64) {
    *value += 1;
}

fn turn_off(value: &mut i64) {
    use std::cmp::max;
    *value = max(0, *value - 1);
}

fn do_nothing(_: &mut i64) {}
