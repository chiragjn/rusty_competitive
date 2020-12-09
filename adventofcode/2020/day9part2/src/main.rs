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

fn solve(lines: Box<dyn Iterator<Item = String>>, n: usize) -> i64 {
    let numbers: Vec<i64> = lines
        .map(|line| {
            line.parse()
                .expect(&format!("Failed to parse {} to i64", line))
        })
        .collect();
    for i in n..numbers.len() {
        let mut ok = false;
        'outer: for j in (i - n)..i {
            for k in (j + 1)..i {
                if numbers[j] != numbers[k] && numbers[j] + numbers[k] == numbers[i] {
                    ok = true;
                    break 'outer;
                }
            }
        }
        if !ok {
            let target = numbers[i];
            let csum: Vec<i64> = (0..1)
                .chain(numbers.iter().scan(0, |acc, &x| {
                    *acc = *acc + x;
                    Some(*acc)
                }))
                .collect();
            for i in 1..csum.len() {
                for j in 0..(i - 1) {
                    if csum[i] - csum[j] == target {
                        return numbers[j..(i - 1)].iter().min().unwrap()
                            + numbers[j..(i - 1)].iter().max().unwrap();
                    }
                }
            }
        }
    }
    unreachable!("Invalid input - atleast one invalid number should exist in the sequence");
}

fn main() {
    let iu = InputUtils::default();
    let boxed_iter = Box::new(iu);
    println!("{}", solve(boxed_iter, 25));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test() {
        let test_input = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it), 5), 62);
    }
}
