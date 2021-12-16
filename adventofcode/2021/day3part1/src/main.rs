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

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut n = 0;
    let mut counts: Vec<u32> = vec![];
    for line in lines {
        n += 1;
        for (i, bit) in line.chars().enumerate() {
            if i >= counts.len() {
                counts.push(0);
            }
            counts[i] += bit.to_digit(10).expect("failed to convert bit to u32");
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, &count) in counts.iter().rev().enumerate() {
        if count > (n - count) {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }
    return gamma * epsilon;
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
        let test_input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 198);
    }
}
