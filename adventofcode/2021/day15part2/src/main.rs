use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::HashMap,
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

fn get_value(grid: &[Vec<u64>], x: usize, y: usize) -> u64 {
    let row_len = grid.len();
    let col_len = grid.last().unwrap().len();
    let tile_i = x / row_len;
    let tile_j = y / col_len;
    let i = x - (row_len * tile_i);
    let j = y - (col_len * tile_j);
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
    let nr = (r * 5) as isize;
    let nc = (c * 5) as isize;
    let mut queue: PriorityQueue<(isize, isize), Reverse<u64>> = PriorityQueue::new();
    let mut distances: HashMap<(isize, isize), u64> = HashMap::new();
    let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    queue.push((0, 0), Reverse(0));
    distances.insert((0, 0), 0);
    while !queue.is_empty() {
        if let Some(((i, j), Reverse(p))) = queue.pop() {
            if i == nr - 1 && j == nc - 1 {
                return p;
            }
            for &(dx, dy) in deltas.iter() {
                let u = i + dx;
                let v = j + dy;
                if u >= 0 && u < nr && v >= 0 && v < nc {
                    let new_p = p + get_value(&grid, u as usize, v as usize);
                    if new_p < *distances.get(&(u, v)).unwrap_or(&u64::MAX) {
                        distances.insert((u, v), new_p);
                        if queue.get_priority(&(u, v)).is_some() {
                            queue.change_priority(&(u, v), Reverse(new_p));
                        } else {
                            queue.push((u, v), Reverse(new_p));
                        }
                    }
                }
            }
        }
    }
    unreachable!("Why are we still here? Just to suffer? The end of the grid should be reachable!");
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
