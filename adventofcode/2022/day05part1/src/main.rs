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

fn move_boxes(columns: &mut [Vec<char>], count: usize, from: usize, to: usize) {
    for _ in 0..count {
        if let Some(c) = columns[from].pop() {
            columns[to].push(c);
        }
    }
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> String {
    let mut columns: Vec<Vec<char>> = vec![];
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        } else {
            for (i, c) in line.chars().enumerate() {
                let idx = (i / 4) as usize;
                while columns.len() <= idx {
                    columns.push(vec![]);
                }
                if ('A'..='Z').contains(&c) {
                    columns[idx].push(c);
                }
            }
        }
    }
    for column in columns.iter_mut() {
        column.reverse();
    }
    let pattern = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    for line in lines {
        let captured = pattern
            .captures(&line)
            .unwrap_or_else(|| panic!("Failed to parse line {:?}", line));
        let count: usize = captured["count"].parse().expect("Failed to cast to int");
        let from: usize = captured["from"].parse().expect("Failed to cast to int");
        let to: usize = captured["to"].parse().expect("Failed to cast to int");
        move_boxes(&mut columns, count, from - 1, to - 1);
    }
    let mut answer: String = String::from("");
    for column in columns {
        if let Some(c) = column.last() {
            answer.push(*c);
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
        let test_input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), "CMZ");
    }
}
