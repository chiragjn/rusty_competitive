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
    let mut pointer: i32 = 50;
    let mut zero_count: u64 = 0;
    for line in lines {
        let direction = line.chars().next().unwrap();
        let mut steps = line[1..].parse::<i32>().unwrap();
        zero_count += (steps / 100) as u64;
        steps %= 100;
        let start = pointer;
        match direction {
            'L' => pointer -= steps,
            'R' => pointer += steps,
            _ => panic!("Invalid direction"),
        }
        while pointer < 0 {
            pointer += 100;
        }
        pointer %= 100;
        if pointer == 0 {
            zero_count += 1u64;
        } else if start != 0 {
            if direction == 'L' && pointer > start {
                zero_count += 1u64;
            } else if direction == 'R' && pointer < start {
                zero_count += 1u64;
            }
        }
    }
    zero_count
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
        let test_input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 6);
    }
}
