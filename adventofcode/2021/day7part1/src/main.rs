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

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut depths: Vec<i64> = lines
        .next()
        .expect("Failed to read input line")
        .split(',')
        .map(|s| s.parse::<i64>().expect("Failed parsing to i64"))
        .collect();
    depths.sort_unstable();
    let mut median: i64 = 0;
    let n = depths.len();
    if n > 0 {
        if n % 2 == 0 {
            median = (depths[(n / 2) - 1] + depths[n / 2]) / 2;
        } else {
            median = depths[n / 2];
        }
    }
    return depths.iter().map(|&d| (d - median).abs()).sum();
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
        assert_eq!(solve(Box::new(it)), 37);
    }
}
