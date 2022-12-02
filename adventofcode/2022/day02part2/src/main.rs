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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

fn to_move(c: char) -> Move {
    match c {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => unreachable!("Unexpected input {}", c),
    }
}

fn to_outcome(c: char) -> Outcome {
    match c {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => unreachable!("Unexpected input {}", c),
    }
}

fn pick(play: Move, outcome: Outcome) -> Move {
    match outcome {
        Outcome::Draw => play,
        Outcome::Lose => match play {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        Outcome::Win => match play {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
    }
}

fn score(turn: String, score_map: &HashMap<(Move, Move), u64>) -> u64 {
    let chars: Vec<char> = turn
        .split_whitespace()
        .into_iter()
        .take(2)
        .map(|part| {
            part.chars()
                .next()
                .expect("got empty string after splitting")
        })
        .collect();
    let mv1 = to_move(chars[0]);
    let o = to_outcome(chars[1]);
    let mv2 = pick(mv1, o);
    *score_map
        .get(&(mv1, mv2))
        .expect("Invalid pair of moves, unable to score")
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let score_map: HashMap<(Move, Move), u64> = HashMap::from([
        ((Move::Paper, Move::Rock), 1),
        ((Move::Scissors, Move::Paper), 2),
        ((Move::Rock, Move::Scissors), 3),
        ((Move::Rock, Move::Rock), 4),
        ((Move::Paper, Move::Paper), 5),
        ((Move::Scissors, Move::Scissors), 6),
        ((Move::Scissors, Move::Rock), 7),
        ((Move::Rock, Move::Paper), 8),
        ((Move::Paper, Move::Scissors), 9),
    ]);
    lines.into_iter().map(|line| score(line, &score_map)).sum()
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
        let test_input = r#"A Y
B X
C Z"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 12);
    }
}
