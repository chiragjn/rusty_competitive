use lazy_static::lazy_static;
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

lazy_static! {
    static ref COUNTERPART: HashMap<char, char> =
        HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    static ref SCORE: HashMap<char, u64> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
}

fn score(line: &str) -> u64 {
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        match c {
            '{' | '<' | '[' | '(' => {
                stack.push(c);
            }
            '}' | '>' | ']' | ')' => {
                if stack.is_empty() || stack.pop().unwrap() != COUNTERPART[&c] {
                    return SCORE[&c];
                }
            }
            _ => {
                unreachable!(format!("invalid character {} in line", c));
            }
        }
    }
    0
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    lines.into_iter().map(|line| score(&line)).sum()
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
        let test_input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 26397);
    }
}
