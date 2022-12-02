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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn convert(play: char) -> Move {
    match play {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => unreachable!("Unexpected input {}", play),
    }
}

fn score(turn: String, score_map: &HashMap<(Move, Move), u64>) -> u64 {
    let moves: Vec<Move> = turn
        .split_whitespace()
        .into_iter()
        .take(2)
        .map(|part| {
            convert(
                part.chars()
                    .next()
                    .expect("got empty string after splitting"),
            )
        })
        .collect();
    *score_map
        .get(&(moves[0], moves[1]))
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
        assert_eq!(solve(Box::new(it)), 15);
    }
}
