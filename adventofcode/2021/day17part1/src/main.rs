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

fn check_collision(vx: i64, vy: i64, x1: i64, x2: i64, y1: i64, y2: i64) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut vx = vx;
    let dx = if vx != 0 { -vx / vx.abs() } else { 0 };
    let dy = -1;
    let cmp = if vx > 0 { std::cmp::max } else { std::cmp::min };
    let mut vy = vy;
    let mut max_y = 0;
    while y >= y1 {
        if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
            return Some(max_y);
        }
        x += vx;
        y += vy;
        max_y = std::cmp::max(max_y, y);
        vx = cmp(vx + dx, 0);
        vy += dy;
    }
    None
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let line = lines.next().expect("failed to read input line!");
    let pattern =
        Regex::new(r"target area: x=(?P<x1>-?\d+)..(?P<x2>-?\d+), y=(?P<y1>-?\d+)..(?P<y2>-?\d+)")
            .unwrap();
    let captured = pattern
        .captures(line.trim())
        .expect("Failed to match to the expected input pattern");
    let x1: i64 = captured["x1"].parse().expect("Failed to cast x1 to i64");
    let x2: i64 = captured["x2"].parse().expect("Failed to cast x2 to i64");
    let y1: i64 = captured["y1"].parse().expect("Failed to cast y1 to i64");
    let y2: i64 = captured["y2"].parse().expect("Failed to cast y2 to i64");
    assert!(x2 > x1);
    assert!(y2 > y1);
    if 0 >= x1 && 0 <= x2 {
        panic!("the target area is directly below or above 0, 0. answer is: INF");
    }
    let mut answer = 0;
    // not the best way to solve this, not generalizable
    for vx in -1000..1000 {
        for vy in -1000..1000 {
            if vx == 0 && vy == 0 {
                continue;
            }
            if let Some(y) = check_collision(vx, vy, x1, x2, y1, y2) {
                answer = std::cmp::max(answer, y);
            }
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
        let test_input = r#"target area: x=20..30, y=-10..-5"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 45);
    }
}
