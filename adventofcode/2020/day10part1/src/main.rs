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
    let mut numbers: Vec<i64> = lines
        .map(|line| {
            line.parse()
                .expect(&format!("Failed to parse {} to i64", line))
        })
        .collect();
    numbers.sort();
    numbers.push(numbers.last().unwrap_or(&0) + 3);
    let mut one_diff = 0;
    let mut three_diff = 0;
    let mut previous = 0;
    for &number in numbers.iter() {
        match number - previous {
            1 => {
                one_diff += 1;
            }
            3 => {
                three_diff += 1;
            }
            _ => {
                panic!(format!(
                    "Invalid input, can't jump from {} -> {}",
                    previous, number
                ));
            }
        }
        previous = number;
    }
    return one_diff * three_diff;
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
        assert_eq!(solve(Box::new(it)), 7 * 5);
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
        assert_eq!(solve(Box::new(it)), 22 * 10);
    }
}
