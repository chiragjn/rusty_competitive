use std::collections::{HashMap, HashSet};
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

fn simulate(active_coords: &mut HashSet<(i32, i32, i32)>) {
    let mut counter: HashMap<(i32, i32, i32), usize> = HashMap::new();
    for &(x, y, z) in active_coords.iter() {
        counter.entry((x, y, z)).or_default();
        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    if !(i == 0 && j == 0 && k == 0) {
                        let key = (x + i, y + j, z + k);
                        *counter.entry(key).or_default() += 1;
                    }
                }
            }
        }
    }
    let mut to_remove = vec![];
    let mut to_add = vec![];
    for (coord, count) in counter {
        if active_coords.contains(&coord) {
            if count < 2 || count > 3 {
                to_remove.push(coord);
            }
        } else {
            if count == 3 {
                to_add.push(coord);
            }
        }
    }
    for coord in to_add {
        active_coords.insert(coord);
    }
    for coord in to_remove {
        active_coords.remove(&coord);
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut active_coords: HashSet<(i32, i32, i32)> = HashSet::new();
    for (i, line) in lines.enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                active_coords.insert((i as i32, j as i32, 0));
            }
        }
    }
    for _ in 0..6 {
        simulate(&mut active_coords);
    }
    return active_coords.len() as i64;
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
        let test_input = r#".#.
..#
###"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 112);
    }
}
