use regex::Regex;
use std::{
    cmp::{max, min},
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
        self.stream.lock().lines().next().map(|line| line.unwrap().trim().to_string())
    }
}

struct LineSegment {
    // ax + by + c = 0
    // (y2 - y1).x + (x1 - x2).y + (x2.y1 - x1.y2)
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let pattern =
        Regex::new(r"(?P<x1>-?\d+),(?P<y1>-?\d+) -> (?P<x2>-?\d+),(?P<y2>-?\d+)").unwrap();
    let mut segments: Vec<LineSegment> = vec![];
    for line in lines {
        let captured = pattern
            .captures(&line)
            .unwrap_or_else(|| panic!("Failed to parse line {:?}", line));
        let x1: i64 = captured["x1"].parse().expect("Failed to cast to int");
        let y1: i64 = captured["y1"].parse().expect("Failed to cast to int");
        let x2: i64 = captured["x2"].parse().expect("Failed to cast to int");
        let y2: i64 = captured["y2"].parse().expect("Failed to cast to int");
        if (x1 == x2) || (y1 == y2) {
            segments.push(LineSegment { x1, y1, x2, y2 });
        }
    }
    let mut point2count: HashMap<(i64, i64), u32> = HashMap::new();
    for segment in segments.iter() {
        if segment.x1 != segment.x2 {
            for x in min(segment.x1, segment.x2)..(max(segment.x1, segment.x2) + 1) {
                *point2count.entry((x, segment.y1)).or_default() += 1;
            }
        } else {
            for y in min(segment.y1, segment.y2)..(max(segment.y1, segment.y2) + 1) {
                *point2count.entry((segment.x1, y)).or_default() += 1;
            }
        }
    }
    return point2count.values().filter(|&&v| v > 1).count() as i64;
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
        let test_input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 5);
    }
}
