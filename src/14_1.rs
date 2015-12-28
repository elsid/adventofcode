extern crate regex;

use regex::Regex;

struct Reindeer {
    pub name: String,
    pub speed: i64,
    pub fly_interval: i64,
    pub rest_interval: i64,
}

struct Parser {
    regex: Regex,
}

fn main() {
    use std::io::{stdin, BufRead};
    let input = stdin();
    let parser = Parser::new();
    let max_distance = input.lock().lines()
        .map(|line| parser.parse(&line.unwrap()[..]))
        .map(|reindeer| get_distance(&reindeer, 2503))
        .max().unwrap();
    println!("{}", max_distance);
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
        regex: Regex::new(
                r"^(?P<name>\w+) can fly (?P<speed>\d+) km/s for (?P<fly_interval>\d+) seconds, but then must rest for (?P<rest_interval>\d+) seconds.$"
            ).unwrap()
        }
    }

    pub fn parse(&self, line: &str) -> Reindeer {
        let captures = self.regex.captures(line).unwrap();
        Reindeer {
            name: captures.name("name").unwrap().to_string(),
            speed: captures.name("speed").unwrap().parse::<i64>().unwrap(),
            fly_interval: captures.name("fly_interval").unwrap().parse::<i64>().unwrap(),
            rest_interval: captures.name("rest_interval").unwrap().parse::<i64>().unwrap(),
        }
    }
}

fn get_distance(reindeer: &Reindeer, interval: i64) -> i64 {
    use std::cmp::min;
    let full = interval / (reindeer.fly_interval + reindeer.rest_interval);
    let last = interval % (reindeer.fly_interval + reindeer.rest_interval);
    (full * reindeer.fly_interval + min(last, reindeer.fly_interval)) * reindeer.speed
}
