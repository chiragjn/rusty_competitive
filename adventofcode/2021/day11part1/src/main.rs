use std::collections::HashSet;
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

fn simulate(grid: &mut Vec<Vec<u8>>) -> u64 {
    let neighbors: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut buffer: Vec<(usize, usize)> = vec![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut flashes = 0;
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = (*cell + 1) % 10;
        }
    }
    loop {
        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 0 && !visited.contains(&(i, j)) {
                    visited.insert((i, j));
                    buffer.push((i, j));
                }
            }
        }
        if buffer.is_empty() {
            break;
        }
        flashes += buffer.len();
        while let Some((i, j)) = buffer.pop() {
            for &(offx, offy) in neighbors.iter() {
                let x = i as isize + offx;
                let y = j as isize + offy;
                if x >= 0 && x < (grid.len() as isize) {
                    let x = x as usize;
                    if y >= 0 && y < (grid[x as usize].len() as isize) {
                        let y = y as usize;
                        if grid[x][y] != 0 {
                            grid[x][y] = (grid[x][y] + 1) % 10;
                        }
                    }
                }
            }
        }
    }
    flashes as u64
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut grid: Vec<Vec<u8>> = vec![];
    for line in lines {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).expect("Failed to cast char to digit") as u8)
                .collect(),
        );
    }
    let mut flashes = 0;
    let days = 100;
    for _ in 0..days {
        flashes += simulate(&mut grid);
    }
    flashes
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
        let test_input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 1656);
    }
}
