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

fn compute(grid: &[Vec<u8>], scenic_score: &mut [Vec<u64>]) {
    let mut buffer: Vec<(u8, usize)> = vec![];
    let x: usize = grid.len();
    let y: usize = grid.first().expect("grid is empty!").len();

    // left to right, multiplies how many trees are visible to the right
    for (i, row) in grid.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            while !buffer.is_empty() && buffer.last().unwrap().0 <= item {
                let idx = buffer.pop().unwrap().1;
                scenic_score[i][idx] *= (j - idx) as u64;
            }
            buffer.push((item, j));
        }
        while !buffer.is_empty() {
            let idx = buffer.pop().unwrap().1;
            scenic_score[i][idx] *= (y - 1 - idx) as u64;
        }
    }

    // right to left
    for (i, row) in grid.iter().enumerate() {
        for (j, &item) in row.iter().enumerate().rev() {
            while !buffer.is_empty() && buffer.last().unwrap().0 <= item {
                let idx = buffer.pop().unwrap().1;
                scenic_score[i][idx] *= (idx - j) as u64;
            }
            buffer.push((item, j));
        }
        while !buffer.is_empty() {
            let idx = buffer.pop().unwrap().1;
            scenic_score[i][idx] *= idx as u64;
        }
    }

    // up to down
    for j in 0..y {
        for (i, row) in grid.iter().enumerate() {
            while !buffer.is_empty() && buffer.last().unwrap().0 <= row[j] {
                let idx = buffer.pop().unwrap().1;
                scenic_score[idx][j] *= (i - idx) as u64;
            }
            buffer.push((row[j], i));
        }
        while !buffer.is_empty() {
            let idx = buffer.pop().unwrap().1;
            scenic_score[idx][j] *= (x - 1 - idx) as u64;
        }
    }

    // down to up
    for j in 0..y {
        for (i, row) in grid.iter().enumerate().rev() {
            while !buffer.is_empty() && buffer.last().unwrap().0 <= row[j] {
                let idx = buffer.pop().unwrap().1;
                scenic_score[idx][j] *= (idx - i) as u64;
            }
            buffer.push((row[j], i));
        }
        while !buffer.is_empty() {
            let idx = buffer.pop().unwrap().1;
            scenic_score[idx][j] *= idx as u64;
        }
    }
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut scenic_score: Vec<Vec<u64>> = vec![];
    for line in lines {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).expect("failed to cast to digit") as u8)
                .collect(),
        );
        scenic_score.push(vec![1; grid.last().expect("grid is empty!").len()]);
    }
    compute(&grid, &mut scenic_score);
    *scenic_score
        .iter()
        .flatten()
        .max()
        .expect("scenic_score is empty!")
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
        assert_eq!(solve(Box::new(it)), 8);
    }
}
