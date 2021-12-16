use std::{
    collections::{HashSet, VecDeque},
    io::{self, BufRead, Stdin},
};

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

fn basin_size(grid: &[Vec<u8>], i: usize, j: usize) -> u64 {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back((i, j));
    visited.insert((i, j));
    let mut size = 0;
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        size += 1;
        if x > 0 && grid[x - 1][y] < 9 && !visited.contains(&(x - 1, y)) {
            queue.push_back((x - 1, y));
            visited.insert((x - 1, y));
        }
        if x < grid.len() - 1 && grid[x + 1][y] < 9 && !visited.contains(&(x + 1, y)) {
            queue.push_back((x + 1, y));
            visited.insert((x + 1, y));
        }
        if y > 0 && grid[x][y - 1] < 9 && !visited.contains(&(x, y - 1)) {
            queue.push_back((x, y - 1));
            visited.insert((x, y - 1));
        }
        if y < grid[x].len() - 1 && grid[x][y + 1] < 9 && !visited.contains(&(x, y + 1)) {
            queue.push_back((x, y + 1));
            visited.insert((x, y + 1));
        }
    }
    size
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut grid: Vec<Vec<u8>> = vec![];
    for line in lines {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).expect("Failed to convert to digit") as u8)
                .collect(),
        );
    }
    let mut basin_sizes: Vec<u64> = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut low = true;
            if i > 0 {
                low &= grid[i][j] < grid[i - 1][j];
            }
            if i < grid.len() - 1 {
                low &= grid[i][j] < grid[i + 1][j];
            }
            if j > 0 {
                low &= grid[i][j] < grid[i][j - 1];
            }
            if j < grid[i].len() - 1 {
                low &= grid[i][j] < grid[i][j + 1];
            }
            if low {
                basin_sizes.push(basin_size(&grid, i, j));
            }
        }
    }
    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    return (&basin_sizes[0..3]).iter().product();
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
        let test_input = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 1134);
    }
}
