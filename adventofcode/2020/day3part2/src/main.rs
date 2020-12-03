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

fn _solve(grid: &Vec<Vec<char>>, right_offset: usize, down_offset: usize) -> i64 {
    let mut x: usize = 0;
    let mut answer = 0;
    if !grid.is_empty() {
        let x_max = grid[0].len();
        for y in (0 + down_offset..grid.len()).step_by(down_offset) {
            x += right_offset;
            x %= x_max;
            if grid[y][x] == '#' {
                answer += 1;
            }
        }
    }
    return answer;
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let grid: Vec<Vec<char>> = lines.map(|s| s.chars().collect()).collect();
    let offsets: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let answer: i64 = offsets
        .iter()
        .map(|&(right_offset, down_offset)| _solve(&grid, right_offset, down_offset))
        .product();
    return answer;
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
        let test_input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 336);
    }
}
