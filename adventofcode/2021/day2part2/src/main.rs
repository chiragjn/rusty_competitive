use regex::Regex;
use std::io::{self, BufRead, Stdin};

struct InputUtils {
    stream: Stdin,
}

impl Default for InputUtils {
    fn default() -> Self {
        return Self {
            stream: io::stdin(),
        };
    }
}

impl Iterator for InputUtils {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.lock().lines().next() {
            Some(line) => Some(line.unwrap().trim().to_string()),
            None => None,
        }
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let pattern = Regex::new(r"(?P<instruction>[a-z]+) (?P<times>\d+)").unwrap();
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut aim: i64 = 0;
    for line in lines {
        let captured = pattern
            .captures(&line)
            .expect(&format!("Failed to parse line {:?}", line));
        let instruction: &str = &captured["instruction"];
        let times: i64 = captured["times"].parse().expect("Failed to cast to int");
        match instruction {
            "forward" => {
                x += times;
                y += aim * times;
            }
            "up" => {
                aim -= times;
            }
            "down" => {
                aim += times;
            }
            _ => {
                unreachable!(format!("Invalid instruction {:?}", instruction));
            }
        }
    }
    return x * y;
}

fn main() {
    let iu = InputUtils::default();
    let boxed_iter = Box::new(iu);
    println!("{}", solve(boxed_iter));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test() {
        let test_input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 900);
    }
}
