use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    io::{self, BufRead, Stdin},
};

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
    static ref ROLLS: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
}

type State = (bool, u8, u8, u8, u8);
type Cache = HashMap<State, u64>;

#[inline]
fn next_pos(current: u8, add: u8) -> u8 {
    (current - 1 + add) % 10 + 1
}

fn _simulate(
    p1_turn: bool,
    p1_pos: u8,
    p2_pos: u8,
    p1_score: u8,
    p2_score: u8,
    winning_score: u8,
    cache: &mut Cache,
) -> u64 {
    let key: State = (p1_turn, p1_pos, p2_pos, p1_score, p2_score);
    if cache.get(&key).is_none() {
        let mut wins = 0;
        if !p1_turn && p1_score >= winning_score {
            wins = 1;
        } else if p1_turn && p2_score >= winning_score {
            wins = 0;
        } else {
            for &(roll, multiplier) in ROLLS.iter() {
                if p1_turn {
                    let new_p1_pos = next_pos(p1_pos, roll);
                    wins += multiplier
                        * _simulate(
                            !p1_turn,
                            new_p1_pos,
                            p2_pos,
                            p1_score + new_p1_pos,
                            p2_score,
                            winning_score,
                            cache,
                        )
                } else {
                    let new_p2_pos = next_pos(p2_pos, roll);
                    wins += multiplier
                        * _simulate(
                            !p1_turn,
                            p1_pos,
                            new_p2_pos,
                            p1_score,
                            p2_score + new_p2_pos,
                            winning_score,
                            cache,
                        )
                }
            }
        }

        cache.insert(key, wins);
    }
    cache[&key]
}

fn run_game(p1_pos: u8, p2_pos: u8, winning_score: u8) -> u64 {
    let mut cache: Cache = Cache::new();
    _simulate(true, p1_pos, p2_pos, 0, 0, winning_score, &mut cache)
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let p1_pos: u8 = lines
        .next()
        .expect("Failed to read player 1's starting position")
        .trim()
        .split(": ")
        .nth(1)
        .expect("Failed to read player 1's starting position")
        .parse()
        .expect("Failed to read player 1's starting position");
    let p2_pos: u8 = lines
        .next()
        .expect("Failed to read player 2's starting position")
        .trim()
        .split(": ")
        .nth(1)
        .expect("Failed to read player 2's starting position")
        .parse()
        .expect("Failed to read player 2's starting position");
    // TODO: we should be able to speed up but using an array cache of 2 x 10 x 10 x 21 x 30
    // and writing a bottom up approach
    run_game(p1_pos, p2_pos, 21)
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
        let test_input = r#"Player 1 starting position: 4
Player 2 starting position: 8"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 444356092776315);
    }
}
