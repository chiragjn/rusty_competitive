use std::collections::{HashSet, VecDeque};
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

fn read_cards(lines: &mut Box<dyn Iterator<Item = String>>) -> VecDeque<usize> {
    lines.next();
    let mut cards: VecDeque<usize> = VecDeque::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        cards.push_back(
            line.parse::<usize>()
                .expect(&format!("failed to cast {} to i64", line)),
        );
    }
    return cards;
}

struct CardGameSimulator {
    a_cards: VecDeque<usize>,
    b_cards: VecDeque<usize>,
    state_cache: HashSet<String>,
}

impl CardGameSimulator {
    fn new(a_cards: VecDeque<usize>, b_cards: VecDeque<usize>) -> Self {
        return Self {
            a_cards: a_cards,
            b_cards: b_cards,
            state_cache: HashSet::new(),
        };
    }

    fn state_hash(&mut self) -> String {
        // crude hashing
        let mut state: String = String::from("1:");
        state = self.a_cards.iter().fold(state, |mut s, v| {
            s.push_str(v.to_string().as_str());
            s.push(',');
            s
        });
        state.push_str("2:");
        state = self.b_cards.iter().fold(state, |mut s, v| {
            s.push_str(v.to_string().as_str());
            s.push(',');
            s
        });
        return state;
    }

    pub fn simulate(&mut self) -> bool {
        while !self.a_cards.is_empty() && !self.b_cards.is_empty() {
            let key = self.state_hash();
            if self.state_cache.contains(&key) {
                return true;
            }
            self.state_cache.insert(key);
            let a_top = self.a_cards.pop_front().unwrap();
            let b_top = self.b_cards.pop_front().unwrap();
            let mut a_won = true;
            if self.a_cards.len() >= a_top && self.b_cards.len() >= b_top {
                a_won = CardGameSimulator::new(
                    self.a_cards.iter().cloned().take(a_top).collect(),
                    self.b_cards.iter().cloned().take(b_top).collect(),
                )
                .simulate();
            } else {
                if b_top > a_top {
                    a_won = false;
                }
            }
            if a_won {
                self.a_cards.push_back(a_top);
                self.a_cards.push_back(b_top);
            } else {
                self.b_cards.push_back(b_top);
                self.b_cards.push_back(a_top);
            }
        }
        return !self.a_cards.is_empty();
    }

    pub fn score(&self) -> u64 {
        let mut winner_deck = &self.a_cards;
        if winner_deck.is_empty() {
            winner_deck = &self.b_cards;
        }
        return winner_deck
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &e)| (i + 1) as u64 * e as u64)
            .sum();
    }
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let a_cards = read_cards(&mut lines);
    let b_cards = read_cards(&mut lines);
    let mut simulator = CardGameSimulator::new(a_cards, b_cards);
    simulator.simulate();
    return simulator.score();
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
        assert_eq!(solve(Box::new(it)), 291);
    }
}
