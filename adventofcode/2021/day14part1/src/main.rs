use std::collections::HashMap;
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

fn react(polymer: Vec<char>, reactions: &HashMap<(char, char), char>) -> Vec<char> {
    if polymer.is_empty() {
        return polymer;
    }
    let mut new_polymer = vec![];
    for (&c1, &c2) in polymer.iter().zip(polymer.iter().skip(1)) {
        new_polymer.push(c1);
        new_polymer.push(reactions[&(c1, c2)]);
    }
    new_polymer.push(*polymer.last().unwrap());
    new_polymer
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut polymer: Vec<char> = vec![];
    for line in &mut lines {
        if line.trim() == "" {
            break;
        }
        polymer = line.trim().chars().collect();
    }
    let mut reactions: HashMap<(char, char), char> = HashMap::new();
    for line in lines {
        let mut parts = line.trim().split(" -> ");
        let pair: Vec<char> = parts
            .next()
            .expect("Failed to read reaction in format AB -> C")
            .chars()
            .collect();
        let ins: char = parts
            .next()
            .and_then(|p| p.chars().next())
            .expect("Failed to read reaction in format AB -> C");
        reactions.insert((pair[0], pair[1]), ins);
    }
    let times = 10;
    for _ in 0..times {
        polymer = react(polymer, &reactions);
    }
    let mut counts: HashMap<char, u64> = HashMap::new();
    for c in polymer {
        *counts.entry(c).or_default() += 1;
    }
    let max_count = *counts.values().max().unwrap_or(&0);
    let min_count = *counts.values().min().unwrap_or(&0);
    max_count - min_count
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
        let test_input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 1588);
    }
}
