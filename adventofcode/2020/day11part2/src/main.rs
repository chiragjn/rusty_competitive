use std::collections::HashMap;
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

struct SeatsSimulator {
    n: isize,
    m: isize,
    previous_state: Vec<Vec<char>>,
    current_state: Vec<Vec<char>>,
    dir_offsets: Vec<(isize, isize)>,
    neighbors: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl SeatsSimulator {
    fn new(state: Vec<Vec<char>>) -> Self {
        return SeatsSimulator {
            n: state.len() as isize,
            m: state.first().unwrap_or(&vec![]).len() as isize,
            previous_state: state.clone(),
            current_state: state,
            dir_offsets: vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ],
            neighbors: HashMap::new(),
        };
    }

    fn compute_neighbors(&mut self, x: usize, y: usize) {
        let key = (x, y);
        if !self.neighbors.contains_key(&key) {
            let ns = self.neighbors.entry(key).or_default();
            for &(off_x, off_y) in self.dir_offsets.iter() {
                let mut xx = x as isize;
                let mut yy = y as isize;
                loop {
                    xx += off_x;
                    yy += off_y;
                    if xx >= 0 && xx < self.n && yy >= 0 && yy < self.m {
                        if self.previous_state[xx as usize][yy as usize] != '.' {
                            ns.push((xx as usize, yy as usize));
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn precompute_neighbors(&mut self) {
        for i in 0..self.previous_state.len() {
            for j in 0..self.previous_state[i].len() {
                if self.previous_state[i][j] != '.' {
                    self.compute_neighbors(i, j);
                }
            }
        }
    }

    fn compute_seat(&mut self, x: usize, y: usize) -> char {
        match self.previous_state[x][y] {
            '.' => {
                return '.';
            }
            'L' => {
                for &(i, j) in self.neighbors[&(x, y)].iter() {
                    if self.previous_state[i][j] == '#' {
                        return 'L';
                    }
                }
                return '#';
            }
            '#' => {
                let mut oc = 0;
                for &(i, j) in self.neighbors[&(x, y)].iter() {
                    if self.previous_state[i][j] == '#' {
                        oc += 1;
                    }
                }
                if oc >= 5 {
                    return 'L';
                }
                return '#';
            }
            _ => {
                panic!(format!(
                    "invalid char {} at {}, {}",
                    self.previous_state[x][y], x, y
                ));
            }
        }
    }

    fn simulate(&mut self) {
        for i in 0..self.previous_state.len() {
            for j in 0..self.previous_state[i].len() {
                self.current_state[i][j] = self.compute_seat(i, j);
            }
        }
    }

    pub fn converge(&mut self) -> i64 {
        self.precompute_neighbors();
        loop {
            self.simulate();
            if self.current_state == self.previous_state {
                break;
            }
            swap(&mut self.previous_state, &mut self.current_state);
        }
        return self
            .current_state
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count() as i64)
            .sum();
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let state: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    return SeatsSimulator::new(state).converge();
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
        assert_eq!(solve(Box::new(it)), 26);
    }
}
