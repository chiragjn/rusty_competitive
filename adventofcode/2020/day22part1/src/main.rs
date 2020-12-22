use std::cmp::Ordering;
use std::collections::VecDeque;
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

fn read_cards(lines: &mut Box<dyn Iterator<Item = String>>) -> VecDeque<i64> {
    lines.next();
    let mut cards: VecDeque<i64> = VecDeque::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        cards.push_back(
            line.parse::<i64>()
                .expect(&format!("failed to cast {} to i64", line)),
        );
    }
    return cards;
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut a_cards = read_cards(&mut lines);
    let mut b_cards = read_cards(&mut lines);
    while !a_cards.is_empty() && !b_cards.is_empty() {
        let a = a_cards.pop_front().unwrap();
        let b = b_cards.pop_front().unwrap();
        match a.cmp(&b) {
            Ordering::Equal => {
                unreachable!("top cards both have equal values, can't proceed!");
            }
            Ordering::Less => {
                b_cards.push_back(b);
                b_cards.push_back(a);
            }
            Ordering::Greater => {
                a_cards.push_back(a);
                a_cards.push_back(b);
            }
        }
    }
    let a_score: i64 = a_cards
        .drain(..)
        .rev()
        .enumerate()
        .map(|(i, e)| (i + 1) as i64 * e)
        .sum();
    let b_score: i64 = b_cards
        .drain(..)
        .rev()
        .enumerate()
        .map(|(i, e)| (i + 1) as i64 * e)
        .sum();
    return a_score + b_score;
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
        let test_input = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 306);
    }
}
