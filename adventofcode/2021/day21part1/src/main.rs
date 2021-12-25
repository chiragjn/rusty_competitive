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

struct Game {
    positions: [u64; 2],
    scores: [u64; 2],
    dice: Box<dyn Iterator<Item = u64>>,
    roll_size: usize,
    roll_count: usize,
    index: usize,
}

impl Game {
    fn new(positions: [u64; 2], dice: Box<dyn Iterator<Item = u64>>, roll_size: usize) -> Self {
        Game {
            positions,
            scores: [0, 0],
            dice,
            roll_size,
            roll_count: 0,
            index: 0,
        }
    }
    fn simulate(&mut self, winning_score: u64) -> u64 {
        loop {
            let roll_sum = self.dice.as_mut().take(self.roll_size).sum::<u64>();
            self.roll_count += 3;
            self.positions[self.index] = (self.positions[self.index] - 1 + roll_sum) % 10 + 1;
            self.scores[self.index] += self.positions[self.index];
            let other = if self.index == 0 { 1 } else { 0 };
            if self.scores[self.index] >= winning_score {
                return self.scores[other] * self.roll_count as u64;
            }
            self.index = other;
        }
    }
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let player1: u64 = lines
        .next()
        .expect("Failed to read player 1's starting position")
        .trim()
        .split(": ")
        .nth(1)
        .expect("Failed to read player 1's starting position")
        .parse()
        .expect("Failed to read player 1's starting position");
    let player2: u64 = lines
        .next()
        .expect("Failed to read player 2's starting position")
        .trim()
        .split(": ")
        .nth(1)
        .expect("Failed to read player 2's starting position")
        .parse()
        .expect("Failed to read player 2's starting position");
    let mut game = Game::new([player1, player2], Box::new((1..=100).cycle()), 3);
    game.simulate(1000)
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
        assert_eq!(solve(Box::new(it)), 739785);
    }
}
