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

struct AddressGenerator {
    mask: Vec<char>,
}

struct AddressGeneratorIterator<'a> {
    mask: &'a Vec<char>,
    max_variants: usize,
    current_variant: usize,
    value: u64,
}

impl AddressGenerator {
    fn new(mask: &str) -> Self {
        return AddressGenerator {
            mask: mask.chars().rev().collect(),
        };
    }

    fn addresses(&self, value: u64) -> AddressGeneratorIterator {
        return AddressGeneratorIterator {
            mask: &self.mask,
            max_variants: 1 << self.mask.iter().filter(|&&c| c == 'X').count(),
            current_variant: 0,
            value: value,
        };
    }
}

impl<'a> Iterator for AddressGeneratorIterator<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_variant >= self.max_variants {
            return None;
        }
        let mut current_variant = self.current_variant;
        let mut generated = 0;
        for (i, &c) in self.mask.iter().enumerate() {
            if c == 'X' {
                if (current_variant & 1) > 0 {
                    generated |= 1 << i;
                }
                current_variant >>= 1;
            } else if c == '1' || (self.value & (1 << i)) > 0 {
                generated |= 1 << i;
            }
        }
        self.current_variant += 1;
        return Some(generated);
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let write_pattern = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
    let mut mask: Option<AddressGenerator> = None;
    for line in lines {
        let mut parts = line.split(" = ");
        let left = parts
            .next()
            .expect(&format!("Failed to get left side on line {}", line));
        let right = parts
            .next()
            .expect(&format!("Failed to get right side on line {}", line));
        if left == "mask" {
            mask = Some(AddressGenerator::new(right));
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
            if let Some(ref smask) = mask.as_ref() {
                for new_address in smask.addresses(address) {
                    memory.insert(new_address, value);
                }
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
        let test_input = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 208);
    }
}
