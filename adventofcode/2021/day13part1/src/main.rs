use std::cmp::max;
use std::io::{self, BufRead, Stdin};

struct InputUtils {
    stream: Stdin,
}

impl Default for InputUtils {
    fn default() -> Self {
        return Self {
            stream: io::stdin(),
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

#[derive(Debug)]
enum FoldLine {
    X(usize),
    Y(usize),
}

fn fold(grid: Vec<Vec<bool>>, fold_line: &FoldLine) -> Vec<Vec<bool>> {
    // yucky impl that rellocates for every fold
    if grid.is_empty() {
        return grid;
    }
    let mut new_grid: Vec<Vec<bool>>;
    let rows = grid.len();
    let cols = grid[0].len();
    match fold_line {
        FoldLine::X(offset) => {
            // vertical line, fold left
            let left: usize = *offset;
            let right = cols - 1 - offset;
            let new_cols = max(left, right);
            new_grid = vec![vec![false; new_cols]; rows];
            for r in 0..rows {
                for (gc, ngc) in (0..=(offset - 1)).rev().zip((0..=(new_cols - 1)).rev()) {
                    new_grid[r][ngc] |= grid[r][gc];
                }
                for (gc, ngc) in ((offset + 1)..=(cols - 1)).zip((0..=(new_cols - 1)).rev()) {
                    new_grid[r][ngc] |= grid[r][gc];
                }
            }
        }
        FoldLine::Y(offset) => {
            // horizontal line, fold up
            let up: usize = *offset;
            let down = rows - 1 - offset;
            let new_rows = max(up, down);
            new_grid = vec![vec![false; cols]; new_rows];
            for c in 0..cols {
                for (gr, ngr) in (0..=(offset - 1)).rev().zip((0..=(new_rows - 1)).rev()) {
                    new_grid[ngr][c] |= grid[gr][c];
                }
                for (gr, ngr) in ((offset + 1)..=(rows - 1)).zip((0..=(new_rows - 1)).rev()) {
                    new_grid[ngr][c] |= grid[gr][c];
                }
            }
        }
    }
    return new_grid;
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut points: Vec<(usize, usize)> = vec![];
    while let Some(line) = lines.next() {
        if line.trim() == "" {
            break;
        }
        let mut parts = line.trim().split(",");
        let x: usize = parts
            .next()
            .and_then(|p| p.parse().ok())
            .expect("Failed to convert x as usize");
        let y: usize = parts
            .next()
            .and_then(|p| p.parse().ok())
            .expect("Failed to convert y as usize");
        points.push((x, y));
    }
    let xmax = points
        .iter()
        .map(|&(x, _)| x)
        .max()
        .expect("No points to take max");
    let ymax = points
        .iter()
        .map(|&(_, y)| y)
        .max()
        .expect("No points to take max");
    let mut grid: Vec<Vec<bool>> = vec![vec![false; xmax + 1]; ymax + 1];
    for &(x, y) in points.iter() {
        grid[y][x] = true;
    }
    let mut fold_lines: Vec<FoldLine> = vec![];
    for line in lines {
        let mut parts = line.trim().split_whitespace();
        parts.next();
        parts.next();
        let mut parts = parts
            .next()
            .and_then(|p| Some(p.split('=')))
            .expect("Third word is not in x/y=\\d+ format");
        let var = parts
            .next()
            .and_then(|p| p.chars().next())
            .expect("Failed to get x/y");
        let along: usize = parts
            .next()
            .and_then(|p| p.parse().ok())
            .expect("Failed to case right side of = to usize");
        if var == 'x' {
            fold_lines.push(FoldLine::X(along));
        } else {
            fold_lines.push(FoldLine::Y(along));
        }
    }
    for fold_line in fold_lines.iter() {
        grid = fold(grid, fold_line);
        break;
    }
    return grid.iter().flatten().filter(|&&b| b).count() as _;
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
        let test_input = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 17);
    }
}
