use num::zero;
use num_traits::{PrimInt, Signed};
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
        self.stream
            .lock()
            .lines()
            .next()
            .map(|line| line.unwrap().trim().to_string())
    }
}

#[derive(Debug)]
struct LineSegment<T: PrimInt + Signed> {
    x1: T,
    y1: T,
    x2: T,
    y2: T,
}

#[derive(Debug)]
struct LineSegmentIterator<'a, T: PrimInt + Signed> {
    segment: &'a LineSegment<T>,
    x: T,
    y: T,
    x_incr: T,
    y_incr: T,
    has_next: bool,
}

impl<'a, T: PrimInt + Signed> LineSegmentIterator<'a, T> {
    fn new(segment: &'a LineSegment<T>) -> Self {
        let mut x_incr = segment.x2 - segment.x1;
        if x_incr != zero::<T>() {
            x_incr = x_incr / x_incr.abs();
        }

        let mut y_incr = segment.y2 - segment.y1;
        if y_incr != zero::<T>() {
            y_incr = y_incr / y_incr.abs();
        }
        LineSegmentIterator {
            segment,
            x: segment.x1,
            y: segment.y1,
            x_incr,
            y_incr,
            has_next: true,
        }
    }
}

impl<'a, T: PrimInt + Signed> Iterator for LineSegmentIterator<'a, T> {
    type Item = (T, T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.has_next {
            let current = Some((self.x, self.y));

            self.x = self.x + self.x_incr;
            self.y = self.y + self.y_incr;
            if self.segment.x1 == self.segment.x2 && self.segment.y1 == self.segment.y2 {
                self.has_next = false;
            }
            if self.x < min(self.segment.x1, self.segment.x2)
                || self.x > max(self.segment.x1, self.segment.x2)
                || self.y < min(self.segment.y1, self.segment.y2)
                || self.y > max(self.segment.y1, self.segment.y2)
            {
                self.has_next = false;
            }

            return current;
        }
        None
    }
}

impl<'a, T: PrimInt + Signed> IntoIterator for &'a LineSegment<T> {
    type Item = (T, T);
    type IntoIter = LineSegmentIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LineSegmentIterator::new(self)
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let pattern =
        Regex::new(r"(?P<x1>-?\d+),(?P<y1>-?\d+) -> (?P<x2>-?\d+),(?P<y2>-?\d+)").unwrap();
    let mut point2count: HashMap<(i64, i64), u32> = HashMap::new();
    for line in lines {
        let captured = pattern
            .captures(&line)
            .unwrap_or_else(|| panic!("Failed to parse line {:?}", line));
        let x1: i64 = captured["x1"].parse().expect("Failed to cast to int");
        let y1: i64 = captured["y1"].parse().expect("Failed to cast to int");
        let x2: i64 = captured["x2"].parse().expect("Failed to cast to int");
        let y2: i64 = captured["y2"].parse().expect("Failed to cast to int");
        let segment = LineSegment { x1, y1, x2, y2 };
        for point in segment.into_iter() {
            *point2count.entry(point).or_default() += 1;
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
        assert_eq!(solve(Box::new(it)), 12);
    }
}
