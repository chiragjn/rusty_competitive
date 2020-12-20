use std::collections::{HashMap, HashSet};
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

struct Tile {
    id: u32,
    data: Vec<Vec<char>>,
}

impl Tile {
    fn all_possible_edges(&self) -> Vec<String> {
        let mut possible_edges: Vec<String> = vec![];
        if !self.data.is_empty() {
            // strong assumption of non-empty 2d grid
            possible_edges.push(self.data.first().unwrap().iter().collect());
            possible_edges.push(self.data.first().unwrap().iter().rev().collect());
            possible_edges.push(self.data.last().unwrap().iter().collect());
            possible_edges.push(self.data.last().unwrap().iter().rev().collect());
            possible_edges.push(self.data.iter().map(|v| v.first().unwrap()).collect());
            possible_edges.push(self.data.iter().rev().map(|v| v.first().unwrap()).collect());
            possible_edges.push(self.data.iter().map(|v| v.last().unwrap()).collect());
            possible_edges.push(self.data.iter().rev().map(|v| v.last().unwrap()).collect());
        }
        return possible_edges;
    }
}

fn read_tile(lines: &mut Box<dyn Iterator<Item = String>>) -> Option<Tile> {
    if let Some(header) = lines.next() {
        let id: u32 = header[5..header.len() - 1].parse().expect(&format!(
            "Failed to get tile id from tile header: {}",
            header,
        ));
        let mut data = vec![];
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            data.push(line.chars().collect());
        }
        return Some(Tile { id: id, data: data });
    }
    return None;
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> i64 {
    let mut tiles: Vec<Tile> = vec![];
    while let Some(tile) = read_tile(&mut lines) {
        tiles.push(tile);
    }
    let mut edge2ids: HashMap<String, HashSet<u32>> = HashMap::new();
    for tile in tiles.iter() {
        for edge in tile.all_possible_edges() {
            edge2ids.entry(edge).or_default().insert(tile.id);
        }
    }
    let mut boundary_ids: HashMap<u32, usize> = HashMap::new();
    for ids in edge2ids.values() {
        if ids.len() == 1 {
            let id = *ids.iter().next().unwrap();
            *boundary_ids.entry(id).or_default() += 1;
        }
    }
    let corner_ids: Vec<u32> = boundary_ids
        .iter()
        .filter(|(_, &v)| v == 4)
        .map(|(&k, _)| k)
        .collect();
    assert_eq!(
        corner_ids.len(),
        4,
        "number of pieces with 2 unique edges != 4: {:?}",
        corner_ids
    );
    return corner_ids.iter().map(|&id| id as i64).product();
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
        let test_input = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 20899048083289);
    }
}
