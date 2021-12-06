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

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let numbers: Vec<usize> = lines
        .next()
        .expect("Failed to read input line")
        .split(',')
        .map(|s| s.parse::<usize>().expect("Failed parsing to usize"))
        .collect();
    const DAYS: usize = 256;
    let mut cache: [u64; DAYS + 20] = [0; DAYS + 20];
    for &number in numbers.iter() {
        cache[number + 1] += 1;
    }
    for i in 1..=DAYS {
        if cache[i] > 0 {
            cache[i + 7] += cache[i];
            cache[i + 9] += cache[i];
        }
    }
    return (numbers.len() as u64) + &cache[1..=DAYS].iter().sum::<u64>();
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
        assert_eq!(solve(Box::new(it)), 26984457539);
    }
}
