use std::io::{self, BufRead, Stdin};

struct InputUtils {
    stream: Stdin,
}

impl Default for InputUtils {
    fn default() -> Self {
        return Self {
            stream: io::stdin(),
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

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u32 {
    let numbers: Vec<i64> = lines
        .map(|s| s.parse::<i64>().expect("Failed parsing to i64"))
        .collect();
    let mut answer: u32 = 0;
    for i in 3..numbers.len() {
        let left = numbers[i - 3] + numbers[i - 2] + numbers[i - 1];
        let right = numbers[i - 2] + numbers[i - 1] + numbers[i];
        if left < right {
            answer += 1;
        }
    }
    return answer;
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
        let test_input = r#"199
200
208
210
200
207
240
269
260
263"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 5);
    }
}
