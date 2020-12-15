use std::collections::HashMap;
use std::io::{self, BufRead, Stdin};

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

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let numbers: Vec<u64> = lines
        .next()
        .expect(&format!("Need a comma separated list of numbers"))
        .split(",")
        .map(|num| {
            num.parse()
                .expect(&format!("failed to cast {} to u64", num))
        })
        .collect();
    let mut memory: HashMap<u64, usize> = HashMap::new();
    let mut previous: u64 = 0;
    let mut current: u64 = 0;
    // take a walk, this takes a while ~2 mins
    for i in 0..30000000 {
        if i < numbers.len() {
            current = numbers[i];
        } else {
            if memory.contains_key(&previous) {
                current = (i - 1 - memory[&previous]) as u64;
            } else {
                current = 0;
            }
        }
        if i > 0 {
            memory.insert(previous, i - 1);
        }
        previous = current;
    }
    return current;
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
        let tests: Vec<(&str, u64)> = vec![
            ("0,3,6", 175594),
            ("1,3,2", 2587),
            ("2,1,3", 3544142),
            ("1,2,3", 261214),
            ("2,3,1", 6895259),
            ("3,2,1", 18),
            ("3,1,2", 362),
        ];
        for (test_input, expected) in tests {
            let it = test_input
                .split('\n')
                .into_iter()
                .map(|part| part.to_string());
            assert_eq!(solve(Box::new(it)), expected);
        }
    }
}
