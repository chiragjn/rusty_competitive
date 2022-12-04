use regex::Regex;
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
    let mut answer = 0;
    let pattern = Regex::new(r"(?P<fl>\d+)-(?P<fr>\d+),(?P<sl>\d+)-(?P<sr>\d+)").unwrap();
    for line in lines {
        let captured = pattern
            .captures(&line)
            .unwrap_or_else(|| panic!("Failed to parse line {:?}", line));
        let fl: u64 = captured["fl"].parse().expect("Failed  to cast to int");
        let fr: u64 = captured["fr"].parse().expect("Failed to cast to int");
        let sl: u64 = captured["sl"].parse().expect("Failed  to cast to int");
        let sr: u64 = captured["sr"].parse().expect("Failed  to cast to int");
        if (fl <= sl && fr >= sr) || (sl <= fl && sr >= fr) {
            answer += 1;
        }
    }
    answer
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
        let test_input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 2);
    }
}
