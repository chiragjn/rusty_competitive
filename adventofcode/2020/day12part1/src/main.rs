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
    let left_turns = ['E', 'N', 'W', 'S'];
    let right_turns = ['E', 'S', 'W', 'N'];
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut direction = 'E';
    for line in lines {
        let mut chars = line.chars();
        let mut ins = chars
            .next()
            .expect(&format!("Failed to parse instruction {}", line));
        let mut off: i64 = chars
            .collect::<String>()
            .parse()
            .expect("Failed to cast offset to i64");
        if ins == 'L' || ins == 'R' {
            if off % 90 != 0 {
                panic!("offsets for L and R instructions must be mutliples of 90");
            }
            off = (off % 360) / 90;
        }
        if ins == 'F' {
            ins = direction;
        }
        match ins {
            'E' => {
                x += off;
            }
            'S' => {
                y -= off;
            }
            'W' => {
                x -= off;
            }
            'N' => {
                y += off;
            }
            'L' => {
                let dir_idx = left_turns.iter().position(|&i| i == direction).unwrap();
                direction = left_turns[(dir_idx + (off as usize)) % 4];
            }
            'R' => {
                let dir_idx = right_turns.iter().position(|&i| i == direction).unwrap();
                direction = right_turns[(dir_idx + (off as usize)) % 4];
            }
            _ => {
                unreachable!(&format!("Invalid instruction: {}", line));
            }
        }
    }
    return x.abs() + y.abs();
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
        let test_input = r#"F10
N3
F7
R90
F11"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 25);
    }
}
