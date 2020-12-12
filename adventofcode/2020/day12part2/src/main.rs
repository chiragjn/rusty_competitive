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

fn rotate_right(x: i64, y: i64) -> (i64, i64) {
    return (y, -x);
}

fn rotate_left(x: i64, y: i64) -> (i64, i64) {
    return (-y, x);
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut way_x: i64 = 10;
    let mut way_y: i64 = 1;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for line in lines {
        let mut chars = line.chars();
        let ins = chars
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
        match ins {
            'F' => {
                x += off * way_x;
                y += off * way_y;
            }
            'E' => {
                way_x += off;
            }
            'S' => {
                way_y -= off;
            }
            'W' => {
                way_x -= off;
            }
            'N' => {
                way_y += off;
            }
            'L' => {
                for _ in 0..off {
                    let temp = rotate_left(way_x, way_y);
                    way_x = temp.0;
                    way_y = temp.1;
                }
            }
            'R' => {
                for _ in 0..off {
                    let temp = rotate_right(way_x, way_y);
                    way_x = temp.0;
                    way_y = temp.1;
                }
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
        assert_eq!(solve(Box::new(it)), 286);
    }
}
