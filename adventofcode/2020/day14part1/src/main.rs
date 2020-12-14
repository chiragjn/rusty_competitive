use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, BufRead, Stdin},
};

struct InputUtils {
    stream: Stdin,
    buffer: String,
}

impl Default for InputUtils {
    fn default() -> Self {
        return Self {
            stream: io::stdin(),
            buffer: String::new(),
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

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let write_pattern = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
    let mut mask: Option<Vec<char>> = None;
    for line in lines {
        let mut parts = line.split(" = ");
        let left = parts
            .next()
            .expect(&format!("Failed to get left side on line {}", line));
        let right = parts
            .next()
            .expect(&format!("Failed to get right side on line {}", line));
        if left == "mask" {
            mask = Some(right.chars().rev().collect());
        } else {
            let captured = write_pattern
                .captures(&line)
                .expect(&format!("Failed to parse line {:?}", line));
            let address: u64 = captured["address"]
                .parse()
                .expect("Failed to cast address to u64");
            let value: u64 = captured["value"]
                .parse()
                .expect("Failed to cast the write value to u64");
            let mut new_value: u64 = 0;
            if let Some(ref smask) = mask.as_ref() {
                for (i, &c) in smask.iter().enumerate() {
                    if c == '1' || (c == 'X' && (value & (1 << i)) > 0) {
                        new_value |= 1 << i;
                    }
                }
                memory.insert(address, new_value);
            }
        }
    }
    return memory.values().sum();
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
        let test_input = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 165);
    }
}
