use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Stdin};
use std::ops::AddAssign;

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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn neighbors(&self) -> [Point<i32>; 6] {
        return [
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
        ];
    }
}

fn simulate(black_tiles: HashSet<Point<i32>>) -> HashSet<Point<i32>> {
    // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
    // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
    let mut counter: HashMap<Point<i32>, usize> = HashMap::new();
    for &black_tile in black_tiles.iter() {
        counter.entry(black_tile).or_default();
        for &neighbor in black_tile.neighbors().iter() {
            *counter.entry(neighbor).or_default() += 1;
        }
    }
    let mut new_black_tiles: HashSet<Point<i32>> = HashSet::new();
    for (point, count) in counter {
        if black_tiles.contains(&point) {
            if !(count == 0 || count > 2) {
                new_black_tiles.insert(point);
            }
        } else {
            if count == 2 {
                new_black_tiles.insert(point);
            }
        }
    }
    return new_black_tiles;
}

fn solve(lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut black_tiles: HashSet<Point<i32>> = HashSet::new();
    for line in lines {
        let mut it = line.chars().enumerate();
        let mut point: Point<i32> = Point { x: 0, y: 0 };
        while let Some((pos, c)) = it.next() {
            match c {
                'n' => match it.next() {
                    Some((_, 'e')) => {
                        point.x += 1;
                        point.y -= 1;
                    }
                    Some((_, 'w')) => {
                        point.y -= 1;
                    }
                    _ => {
                        unreachable!(
                            "invalid/missing char after 'n' at pos {} in line {}",
                            pos, line
                        );
                    }
                },
                's' => match it.next() {
                    Some((_, 'e')) => {
                        point.y += 1;
                    }
                    Some((_, 'w')) => {
                        point.x -= 1;
                        point.y += 1;
                    }
                    _ => {
                        unreachable!(
                            "invalid/missing char after 's' at pos {} in line {}",
                            pos, line
                        );
                    }
                },
                'e' => {
                    point.x += 1;
                }
                'w' => {
                    point.x += -1;
                }
                _ => {
                    unreachable!("invalid char {} at position {} in line {}", c, pos, line);
                }
            }
        }
        if black_tiles.contains(&point) {
            black_tiles.remove(&point);
        } else {
            black_tiles.insert(point);
        }
    }
    for _ in 0..100 {
        black_tiles = simulate(black_tiles);
    }
    return black_tiles.len() as i64;
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
        let test_input = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 2208);
    }
}
