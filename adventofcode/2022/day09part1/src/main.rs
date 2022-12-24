use std::{
    collections::HashSet,
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Location {
    x: i64,
    y: i64,
}

fn move_location(location: &mut Location, direction: char) {
    match direction {
        'U' => {
            location.y += 1;
        }
        'D' => {
            location.y -= 1;
        }
        'R' => {
            location.x += 1;
        }
        'L' => {
            location.x -= 1;
        }
        _ => unreachable!("direction must be one of [U, D, R, L]"),
    }
}

fn adjust_tail(head: &Location, tail: &mut Location) {
    if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
        return;
    }
    if (head.y - tail.y).abs() == 0 {
        move_location(tail, if head.x > tail.x { 'R' } else { 'L' });
        return;
    }
    if (head.x - tail.x).abs() == 0 {
        move_location(tail, if head.y > tail.y { 'U' } else { 'D' });
        return;
    }
    if (head.y - tail.y).abs() == 1 {
        move_location(tail, if head.y > tail.y { 'U' } else { 'D' });
        adjust_tail(head, tail);
        return;
    }
    move_location(tail, if head.x > tail.x { 'R' } else { 'L' });
    adjust_tail(head, tail);
}

fn simulate(moves: &[(char, usize)]) -> u64 {
    let mut head = Location { x: 0, y: 0 };
    let mut tail = Location { x: 0, y: 0 };
    let mut tail_positions: HashSet<Location> = HashSet::new();
    tail_positions.insert(tail);
    for &(direction, units) in moves {
        for _ in 0..units {
            move_location(&mut head, direction);
            adjust_tail(&head, &mut tail);
            tail_positions.insert(tail);
        }
    }
    tail_positions.len() as u64
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut moves: Vec<(char, usize)> = vec![];
    for line in lines {
        let mut it = line.split_whitespace();
        let direction = it
            .next()
            .and_then(|part| part.chars().next())
            .expect("expected a direction character but got empty line");
        let count = it
            .next()
            .and_then(|part| part.parse::<usize>().ok())
            .expect("expected a integer after a direction");
        moves.push((direction, count));
    }
    simulate(&moves)
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
        let test_input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 13);
    }
}
