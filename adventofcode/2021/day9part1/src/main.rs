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

fn solve(lines: Box<dyn Iterator<Item = String>>) -> u32 {
    let mut grid: Vec<Vec<u8>> = vec![];
    for line in lines {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).expect("Failed to convert to digit") as u8)
                .collect(),
        );
    }
    let mut answer = 0;
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
                answer += 1 + (grid[i][j] as u32);
            }
        }
    }
    answer
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
        assert_eq!(solve(Box::new(it)), 15);
    }
}
