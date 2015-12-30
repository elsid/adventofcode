extern crate regex;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use regex::{Regex, Captures};

type Variables = HashMap<String, Box<Fn() -> u16>>;
type VariablesArc = Arc<RwLock<Variables>>;
type Cache = HashMap<String, u16>;
type CacheArc = Arc<RwLock<Cache>>;

struct Parser {
    variables: VariablesArc,
    cache: CacheArc,
    assign: Regex,
    unary: Regex,
    binary: Regex,
}

fn main() {
    use std::io::{stdin, BufRead};
    let input = stdin();
    let variables = Arc::new(RwLock::new(Variables::new()));
    let cache = Arc::new(RwLock::new(Cache::new()));
    let parser = Parser::new(variables.clone(), cache.clone());
    for line in input.lock().lines() {
        let (target, function) = parser.parse(&line.unwrap()[..]);
        variables.write().unwrap().insert(target, function);
    }
    let a = variables.read().unwrap()["a"]();
    variables.write().unwrap().insert("b".to_string(), Box::new(move || a));
    cache.write().unwrap().clear();
    let result = variables.read().unwrap()["a"]();
    println!("{}", result);
}

impl Parser {
    pub fn new(variables: VariablesArc, cache: CacheArc) -> Parser {
        Parser {
            variables: variables,
            cache: cache,
            assign: Regex::new(
                r"^(?P<value>\w+) -> (?P<target>\w+)$"
            ).unwrap(),
            unary: Regex::new(
                r"^(?P<operation>NOT) (?P<value>\w+) -> (?P<target>\w+)$"
            ).unwrap(),
            binary: Regex::new(
                r"^(?P<lhs>\w+) (?P<operation>AND|OR|LSHIFT|RSHIFT) (?P<rhs>\w+) -> (?P<target>\w+)$"
            ).unwrap()
        }
    }

    pub fn parse(&self, line: &str) -> (String, Box<Fn() -> u16>) {
        self.try_assign(line)
    }

    fn try_assign(&self, line: &str) -> (String, Box<Fn() -> u16>) {
        match self.assign.captures(line) {
            Some(captures) => self.on_assign(captures),
            None => self.try_unary(line),
        }
    }

    fn try_unary(&self, line: &str) -> (String, Box<Fn() -> u16>) {
        match self.unary.captures(line) {
            Some(captures) => self.on_unary(captures),
            None => self.try_binary(line),
        }
    }

    fn try_binary(&self, line: &str) -> (String, Box<Fn() -> u16>) {
        match self.binary.captures(line) {
            Some(captures) => self.on_binary(captures),
            None => panic!(),
        }
    }

    fn on_assign(&self, captures: Captures) -> (String, Box<Fn() -> u16>) {
        let value = captures.name("value").unwrap();
        let target = captures.name("target").unwrap();
        (target.to_string(), self.make_get_function(value))
    }

    fn on_unary(&self, captures: Captures) -> (String, Box<Fn() -> u16>) {
        let value = captures.name("value").unwrap();
        let operation = captures.name("operation").unwrap();
        let target = captures.name("target").unwrap();
        let get_value = self.make_get_function(value);
        let function: Box<Fn() -> u16> = match operation {
            "NOT" => Box::new(move || !get_value()),
            _ => panic!(),
        };
        (target.to_string(), function)
    }

    fn on_binary(&self, captures: Captures) -> (String, Box<Fn() -> u16>) {
        let lhs = captures.name("lhs").unwrap();
        let rhs = captures.name("rhs").unwrap();
        let operation = captures.name("operation").unwrap();
        let target = captures.name("target").unwrap();
        let get_lhs = self.make_get_function(lhs);
        let get_rhs = self.make_get_function(rhs);
        let function: Box<Fn() -> u16> = match operation {
            "AND" => Box::new(move || get_lhs() & get_rhs()),
            "OR" => Box::new(move || get_lhs() | get_rhs()),
            "LSHIFT" => Box::new(move || get_lhs() << get_rhs()),
            "RSHIFT" => Box::new(move || get_lhs() >> get_rhs()),
            _ => panic!(),
        };
        (target.to_string(), function)
    }

    fn make_get_function(&self, value: &str) -> Box<Fn() -> u16> {
        let result: Box<Fn() -> u16> = match value.parse::<u16>() {
            Ok(number) => Box::new(move || number),
            Err(_) => {
                let variables = self.variables.clone();
                let cache = self.cache.clone();
                let name = value.to_string();
                Box::new(move || {
                    let cached = match cache.read().unwrap().get(&name) {
                        Some(&value) => Some(value),
                        None => None,
                    };
                    match cached {
                        Some(value) => value,
                        None => {
                            let value = variables.read().unwrap()[&name]();
                            cache.write().unwrap().insert(name.clone(), value);
                            value
                        }
                    }
                })
            },
        };
        result
    }
}
