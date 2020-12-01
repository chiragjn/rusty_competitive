use std::io::{self, BufRead, Stdin};

struct InputUtils {
    stream: Stdin,
    buffer: String,
}

impl Default for InputUtils {
    fn default() -> Self {
        return Self {
            stream: io::stdin(),
            buffer: String::new(),
        };
    }
}

impl Iterator for InputUtils {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.lock().lines().next() {
            Some(line) => Some(line.unwrap().trim().to_string()),
            None => None,
        }
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let numbers: Vec<i64> = lines
        .map(|s| s.parse::<i64>().expect("Failed parsing to i64"))
        .collect();
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return numbers[i] * numbers[j];
            }
        }
    }
    unreachable!("Invalid input, no two numbers add up to 2020");
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
        let test_input = r#"1721
979
366
299
675
1456"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 514579);
    }
}
