use std::collections::HashSet;
use std::io::{self, BufRead, Stdin};
use std::iter::FromIterator;

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

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut collected: HashSet<char> = HashSet::new();
    let mut answer = 0;
    let mut in_progress = false;
    for line in lines {
        if line.is_empty() {
            answer += collected.len();
            collected.clear();
            in_progress = false;
        } else {
            if !in_progress {
                collected = HashSet::from_iter(line.chars());
            } else {
                let temp = HashSet::<char>::from_iter(line.chars());
                collected.retain(|x| temp.contains(x));
            }
            in_progress = true;
        }
    }
    answer += collected.len();
    return answer as i64;
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
        let test_input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 6);
    }
}
