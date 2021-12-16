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
        self.stream.lock().lines().next().map(|line| line.unwrap().trim().to_string())
    }
}

fn get_value(grid: &Vec<Vec<u64>>, x: usize, y: usize) -> u64 {
    let r = grid.len();
    let c = grid.last().unwrap().len();
    let tile_i = x / r;
    let tile_j = y / c;
    let i = x - (r * tile_i);
    let j = y - (c * tile_j);
    ((grid[i][j] + (tile_i as u64) + (tile_j as u64) - 1) % 9) + 1
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
    let nr = r * 5;
    let nc = c * 5;
    let mut cache: Vec<Vec<u64>> = vec![vec![0; nc]; 2];
    let mut G = vec![];
    for i in (0..nr).rev() {
        for j in (0..nc).rev() {
            let value = get_value(&grid, i, j);
            if i == nr - 1 && j == nc - 1 {
                cache[0][j] = value;
                continue;
            }
            let right = if j < nc - 1 && cache[0][j + 1] < u64::MAX {
                value + cache[0][j + 1]
            } else {
                u64::MAX
            };
            let down = if i < nr - 1 && cache[1][j] < u64::MAX {
                value + cache[1][j]
            } else {
                u64::MAX
            };
            cache[0][j] = std::cmp::min(right, down);
        }
        G.push(cache[0].clone());
        cache.swap(0, 1);
    }
    G.reverse();
    for i in 0..nr {
        println!("{:?} ... {:?}", &G[i][0..5], &G[i][(nc - 5)..nc]);
    }
    cache[1][0] - grid[0][0]
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
        assert_eq!(solve(Box::new(it)), 315);
    }
}
