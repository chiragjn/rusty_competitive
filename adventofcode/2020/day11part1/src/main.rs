use std::io::{self, BufRead, Stdin};
use std::mem::swap;

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

fn convolve(state: &Vec<Vec<char>>, x: usize, y: usize, n: usize) -> char {
    match state[x][y] {
        '.' => {
            return '.';
        }
        'L' => {
            for i in (x - n)..(x + n + 1) {
                for j in (y - n)..(y + n + 1) {
                    if state[i][j] == '#' {
                        return 'L';
                    }
                }
            }
            return '#';
        }
        '#' => {
            let mut oc = -1;
            for i in (x - n)..(x + n + 1) {
                for j in (y - n)..(y + n + 1) {
                    if state[i][j] == '#' {
                        oc += 1;
                    }
                }
            }
            if oc >= 4 {
                return 'L';
            }
            return '#';
        }
        _ => {
            panic!(format!("invalid char {} at {}, {}", state[x][y], x, y));
        }
    }
}

fn simulate(previous_state: &Vec<Vec<char>>, current_state: &mut Vec<Vec<char>>) {
    // should be able to parallelize this pretty easy!
    for i in 1..previous_state.len() - 1 {
        for j in 1..previous_state[i].len() - 1 {
            current_state[i][j] = convolve(previous_state, i, j, 1);
        }
    }
}

fn converge(previous_state: &mut Vec<Vec<char>>, current_state: &mut Vec<Vec<char>>) {
    loop {
        simulate(&previous_state, current_state);
        if current_state == previous_state {
            break;
        }
        swap(previous_state, current_state);
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut previous_state: Vec<Vec<char>> = vec![];
    for (i, line) in lines.enumerate() {
        previous_state.push(vec!['.']);
        previous_state[i].extend(line.chars());
        previous_state[i].push('.');
    }
    if previous_state.is_empty() {
        return 0;
    }
    previous_state.insert(0, vec!['.'; previous_state[0].len()]);
    previous_state.push(vec!['.'; previous_state[0].len()]);
    let mut current_state = previous_state.clone();
    converge(&mut previous_state, &mut current_state);
    return current_state
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count() as i64)
        .sum();
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
        let test_input = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 37);
    }
}
