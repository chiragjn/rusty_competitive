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
    let mut numbers: Vec<i64> = vec![0];
    numbers.extend(lines.map(|line| {
        line.parse::<i64>()
            .expect(&format!("Failed to parse {} to i64", line))
    }));
    numbers.sort();
    numbers.push(numbers.last().unwrap_or(&0) + 3);
    let mut cache = vec![0; numbers.len()];
    cache[numbers.len() - 1] = 1;
    for i in (0..numbers.len() - 1).rev() {
        for j in i + 1..numbers.len() {
            if numbers[j] - numbers[i] <= 3 {
                cache[i] += cache[j];
            } else {
                break;
            }
        }
    }
    return cache[0];
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
    fn test_small() {
        let test_input = r#"16
10
15
5
1
11
7
19
6
12
4"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 8);
    }

    #[test]
    fn test_large() {
        let test_input = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 19208);
    }
}
