use std::collections::HashSet;
use std::io::{self, BufRead, Stdin};

struct InputUtils {
    stream: Stdin,
}

impl Default for InputUtils {
    fn default() -> Self {
        Self {
            stream: io::stdin(),
        }
    }
}

impl Iterator for InputUtils {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream
            .lock()
            .lines()
            .next()
            .map(|line| line.unwrap().trim().to_string())
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut sum: u64 = 0;
    for line in lines {
        let len = line.len();
        let first: HashSet<char> = HashSet::from_iter(line.chars().take(len / 2));
        let second: HashSet<char> = HashSet::from_iter(line.chars().skip(len / 2));
        let common = *first
            .intersection(&second)
            .into_iter()
            .next()
            .expect("no common item found!");
        if common.is_lowercase() {
            sum += 1 + common as u64 - 'a' as u64
        } else {
            sum += 27 + common as u64 - 'A' as u64;
        }
    }
    sum
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
        let test_input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 157);
    }
}
