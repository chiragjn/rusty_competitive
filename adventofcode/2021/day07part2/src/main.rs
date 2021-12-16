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

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut depths: Vec<i64> = lines
        .next()
        .expect("Failed to read input line")
        .split(',')
        .map(|s| s.parse::<i64>().expect("Failed parsing to i64"))
        .collect();
    depths.sort_unstable();
    // how do we solve this analytically?
    let mut answer: i64 = i64::MAX;
    let low = *depths.first().unwrap();
    let high = *depths.last().unwrap();
    for x in low..=high {
        let mut cost: i64 = 0;
        for d in depths.iter() {
            let n = (d - x).abs();
            cost += (n * (n + 1)) / 2;
        }
        answer = answer.min(cost);
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
        let test_input = r#"16,1,2,0,4,2,7,1,2,14"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 168);
    }
}
