use std::collections::HashMap;
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

fn react(
    polymer_state: HashMap<(char, char), u64>,
    reactions: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    if polymer_state.is_empty() {
        return polymer_state;
    }
    let mut new_polymer_state = HashMap::new();
    let mut ins: char;
    for (&(c1, c2), &count) in polymer_state.iter() {
        ins = reactions[&(c1, c2)];
        *new_polymer_state.entry((c1, ins)).or_default() += count;
        *new_polymer_state.entry((ins, c2)).or_default() += count;
    }
    return new_polymer_state;
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut polymer: Vec<char> = vec![];
    let mut polymer_state: HashMap<(char, char), u64> = HashMap::new();
    while let Some(line) = lines.next() {
        if line.trim() == "" {
            break;
        }
        polymer = line.trim().chars().collect();
        for (&c1, &c2) in polymer.iter().zip(polymer.iter().skip(1)) {
            *polymer_state.entry((c1, c2)).or_default() += 1;
        }
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
    let times = 40;
    for _ in 0..times {
        polymer_state = react(polymer_state, &reactions);
    }
    let mut counts: HashMap<char, u64> = HashMap::new();
    for (&(c1, _), &count) in polymer_state.iter() {
        *counts.entry(c1).or_default() += count;
    }
    if !polymer.is_empty() {
        *counts.entry(*polymer.last().unwrap()).or_default() += 1;
    }
    let max_count = *counts.values().max().unwrap_or(&0);
    let min_count = *counts.values().min().unwrap_or(&0);
    return max_count - min_count;
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
        assert_eq!(solve(Box::new(it)), 2188189693529);
    }
}
