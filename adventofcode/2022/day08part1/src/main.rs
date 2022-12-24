use std::{
    cmp::max,
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

fn vertical_sweep(grid: &[Vec<u8>], visible: &mut [Vec<u8>], reverse: bool) {
    let x: usize = grid.len();
    let y: usize = grid.first().expect("grid is empty!").len();
    let mut buffer: Vec<u8> = vec![0; y];
    let mut it: Box<dyn DoubleEndedIterator<Item = usize>> = Box::new(0..x);
    if reverse {
        it = Box::new(it.rev());
    }
    for i in it {
        for (j, v) in buffer.iter_mut().enumerate().take(y) {
            if i == 0 || j == 0 || i == x - 1 || j == y - 1 || grid[i][j] > *v {
                visible[i][j] = 1;
            }
            *v = max(*v, grid[i][j]);
        }
    }
}

fn horizontal_sweep(grid: &[Vec<u8>], visible: &mut [Vec<u8>], reverse: bool) {
    let x: usize = grid.len();
    let y: usize = grid.first().expect("grid is empty!").len();
    let mut buffer: Vec<u8> = vec![0; x];
    let mut it: Box<dyn DoubleEndedIterator<Item = usize>> = Box::new(0..y);
    if reverse {
        it = Box::new(it.rev());
    }
    for j in it {
        for (i, v) in buffer.iter_mut().enumerate().take(x) {
            if i == 0 || j == 0 || i == x - 1 || j == y - 1 || grid[i][j] > *v {
                visible[i][j] = 1;
            }
            *v = max(*v, grid[i][j]);
        }
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut visible: Vec<Vec<u8>> = vec![];
    for line in lines {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).expect("failed to cast to digit") as u8)
                .collect(),
        );
        visible.push(vec![0; grid.last().expect("grid is empty!").len()]);
    }
    vertical_sweep(&grid, &mut visible, false);
    vertical_sweep(&grid, &mut visible, true);
    horizontal_sweep(&grid, &mut visible, false);
    horizontal_sweep(&grid, &mut visible, true);
    visible.iter().flatten().map(|&x| x as u64).sum()
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
        let test_input = r#"30373
25512
65332
33549
35390"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 21);
    }
}
