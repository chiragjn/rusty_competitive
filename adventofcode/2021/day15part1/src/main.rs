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
    let mut grid: Vec<Vec<u64>> = vec![];
    for line in lines {
        grid.push(
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("Failed to cast to digit") as _)
                .collect(),
        );
    }
    if grid.is_empty() || grid.last().unwrap().is_empty() {
        return 0;
    }
    let r = grid.len();
    let c = grid.last().unwrap().len();
    let mut cache: Vec<Vec<u64>> = vec![vec![0; c]; r];
    cache[r - 1][c - 1] = grid[r - 1][c - 1];
    for i in (0..r).rev() {
        for j in (0..c).rev() {
            if i == r - 1 && j == c - 1 {
                continue;
            }
            let right = if j < c - 1 && cache[i][j + 1] < u64::MAX {
                grid[i][j] + cache[i][j + 1]
            } else {
                u64::MAX
            };
            let down = if i < r - 1 && cache[i + 1][j] < u64::MAX {
                grid[i][j] + cache[i + 1][j]
            } else {
                u64::MAX
            };
            cache[i][j] = std::cmp::min(right, down);
        }
    }

    cache[0][0] - grid[0][0]
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
        let test_input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 40);
    }
}
