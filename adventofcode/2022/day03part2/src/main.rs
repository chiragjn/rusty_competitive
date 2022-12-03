use itertools::Itertools;
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
    for chunk in &lines.chunks(3) {
        let sets: Vec<HashSet<char>> = chunk
            .map(|line| HashSet::<char>::from_iter(line.chars()))
            .collect();
        let slice = &sets;
        let common = *slice[0]
            .iter()
            .filter(move |c| slice[1..].iter().all(|s| s.contains(c)))
            .take(1)
            .next()
            .expect("no common character found");
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
        assert_eq!(solve(Box::new(it)), 70);
    }
}
