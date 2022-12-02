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

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut cals: Vec<u64> = vec![];
    let mut running_cal: u64 = 0;
    for line in lines {
        if line.trim().is_empty() {
            cals.push(running_cal);
            running_cal = 0;
        }
        else {
            running_cal += line.parse::<u64>().expect("Failed parsing to u64");
        }
    }
    cals.push(running_cal);
    cals.sort();
    cals.iter().rev().take(3).sum()
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
        let test_input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 45000);
    }
}