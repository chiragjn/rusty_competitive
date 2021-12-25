use std::{
    fmt::Display,
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

#[derive(Debug, PartialEq, Eq)]
enum LitPixels {
    Infinite,
    Finite(u64),
}

impl Display for LitPixels {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Infinite => formatter.write_str("inf"),
            Self::Finite(x) => formatter.write_str(&format!("{}", x)),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Background {
    Light,
    Dark,
}

struct Picture {
    pixels: [Vec<Vec<bool>>; 2],
    background: Background,
}

impl Picture {
    fn new(grid: Vec<Vec<char>>, times: usize) -> Self {
        let n = grid.len();
        let m = if n > 0 { grid[0].len() } else { 0 };
        let mut pixels = [
            vec![vec![false; m + times + 1 + times + 1]; n + times + 1 + times + 1],
            vec![vec![false; m + times + 1 + times + 1]; n + times + 1 + times + 1],
        ];
        for (i, x) in (0..n).zip(times + 1..) {
            for (j, y) in (0..m).zip(times + 1..) {
                pixels[0][x][y] = grid[i][j] == '#';
            }
        }
        Picture {
            pixels,
            background: Background::Dark,
        }
    }

    fn enhance(&mut self, emap: &[bool]) {
        if self.background == Background::Dark && emap[0] {
            self.background = Background::Light;
        } else if self.background == Background::Light && !emap[511] {
            self.background = Background::Dark;
        }
        for i in 0..(self.pixels[0].len()) {
            for j in 0..(self.pixels[0][0].len()) {
                if i == 0
                    || j == 0
                    || i == self.pixels[0].len() - 1
                    || j == self.pixels[0][0].len() - 1
                {
                    self.pixels[1][i][j] = match self.background {
                        Background::Dark => false,
                        Background::Light => true,
                    };
                } else {
                    let mut index: usize = 0;
                    for x in -1..=1 {
                        for y in -1..=1 {
                            let a = (i as isize + x) as usize;
                            let b = (j as isize + y) as usize;
                            index = (index << 1) | self.pixels[0][a][b] as usize;
                        }
                    }
                    self.pixels[1][i][j] = emap[index];
                }
            }
        }
        self.pixels.swap(0, 1);
    }

    #[allow(dead_code)]
    fn display(&self) {
        for row in self.pixels[0].iter() {
            println!(
                "{}",
                row.iter()
                    .map(|&c| if c { '#' } else { '.' })
                    .collect::<String>()
            );
        }
    }
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> LitPixels {
    let emap: Vec<bool> = lines
        .next()
        .expect("failed to read the enhancement mapping")
        .chars()
        .map(|c| c == '#')
        .collect();
    lines.next();
    let grid: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let times = 50;
    let mut picture = Picture::new(grid, times);
    for _ in 0..times {
        picture.enhance(&emap);
    }
    if picture.background == Background::Light {
        return LitPixels::Infinite;
    }
    LitPixels::Finite(
        picture.pixels[0]
            .iter()
            .flatten()
            .map(|&c| c as u64)
            .sum::<u64>(),
    )
}

fn main() {
    let iu = InputUtils::default();
    let boxed_iter = Box::new(iu);
    println!("{}", solve(boxed_iter));
}

#[cfg(test)]
mod tests {
    use crate::{solve, LitPixels};

    #[test]
    fn test() {
        let test_input = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), LitPixels::Finite(3351));
    }
}
