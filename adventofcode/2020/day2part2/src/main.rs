use regex::Regex;
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

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut answer = 0;
    let pattern =
        Regex::new(r"(?P<low>\d+)-(?P<high>\d+) (?P<ch>[a-z]): (?P<password>[a-z]+)").unwrap();
    for line in lines {
        let captured = pattern
            .captures(&line)
            .expect(&format!("Failed to parse line {:?}", line));
        let low: usize = captured["low"].parse().expect("Failed  to cast to int");
        let high: usize = captured["high"].parse().expect("Failed to cast to int");
        let ch: char = captured["ch"]
            .chars()
            .next()
            .expect("Failed to get character after range");
        let chars: Vec<char> = captured["password"].chars().collect();
        if chars[low - 1] != chars[high - 1] && (chars[low - 1] == ch || chars[high - 1] == ch) {
            answer += 1;
        }
    }
    return answer;
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
        let test_input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 1);
    }
}
