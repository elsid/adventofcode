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
    pub action: fn(&mut bool) -> (),
    pub line: Line,
}

struct Grid {
    values: Vec<bool>,
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
    let count = grid.values().iter().filter(|&&v| v).count();
    println!("{}", count);
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            regex: Regex::new(r"(?P<action>toggle|turn on|turn off) (?P<begin_x>\d+),(?P<begin_y>\d+) through (?P<end_x>\d+),(?P<end_y>\d+)").unwrap()
        }
    }

    pub fn parse(&self, line: &str) -> Area {
        let captures = self.regex.captures(line).unwrap();
        let action: fn(&mut bool) -> () = match captures.name("action").unwrap() {
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
        Grid {values: (0..1000*1000).map(|_| false).collect::<_>()}
    }

    pub fn apply(&mut self, area: &Area) {
        for x in area.line.begin.x..area.line.end.x + 1 {
            for y in area.line.begin.y..area.line.end.y + 1 {
                let action = area.action;
                action(&mut self.values[x + y * 1000]);
            }
        }
    }

    pub fn values(&self) -> &Vec<bool> {
        &self.values
    }
}

fn toggle(value: &mut bool) {
    *value = !*value;
}

fn turn_on(value: &mut bool) {
    *value = true;
}

fn turn_off(value: &mut bool) {
    *value = false;
}

fn do_nothing(_: &mut bool) {}
