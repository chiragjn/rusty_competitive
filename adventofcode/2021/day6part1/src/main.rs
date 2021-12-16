use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
        self.stream.lock().lines().next().map(|line| line.unwrap().trim().to_string())
    }
}

fn simulate(initials: &Vec<u64>, last_day: u64) -> u64 {
    let mut pq: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
    for initial in initials {
        pq.push(Reverse(initial + 1));
    }
    loop {
        if let Some(Reverse(x)) = pq.pop() {
            if x > last_day {
                pq.push(Reverse(x));
                break;
            }
            pq.push(Reverse(x + 7));
            pq.push(Reverse(x + 9));
        }
    }
    pq.len() as u64
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let numbers: Vec<u64> = lines
        .next()
        .expect("Failed to read input line")
        .split(',')
        .map(|s| s.parse::<u64>().expect("Failed parsing to u64"))
        .collect();
    let period = 80;
    simulate(&numbers, period)
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
        let test_input = r#"3,4,3,1,2"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 5934);
    }
}
