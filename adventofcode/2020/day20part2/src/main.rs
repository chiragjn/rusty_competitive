// very brute solution because small data
use std::collections::{HashMap, HashSet, VecDeque};
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

#[derive(Debug)]
struct Tile {
    id: u32,
    data: Vec<Vec<char>>,
    resolved: bool,
    neighbors: [Option<Option<u32>>; 4], // U, D, L, R
}

impl Tile {
    fn new(id: u32, data: Vec<Vec<char>>) -> Self {
        return Self {
            id: id,
            data: data,
            resolved: false,
            neighbors: [None, None, None, None],
        };
    }

    fn mark_resolved(&mut self) {
        if self.neighbors.iter().any(|i| i.is_none()) {
            panic!(
                "Cannot mark tile {:?} as resolved as some neighbors are None",
                self
            );
        }
        self.resolved = true;
    }

    fn flip_horizontally(&mut self) {
        self.data.reverse();
    }

    fn rotate_clockwise(&mut self) {
        let n = self.data.len();
        for i in 0..(n / 2) {
            for j in i..(n - i - 1) {
                let temp = self.data[i][j];
                self.data[i][j] = self.data[n - 1 - j][i];
                self.data[n - 1 - j][i] = self.data[n - 1 - i][n - 1 - j];
                self.data[n - 1 - i][n - 1 - j] = self.data[j][n - 1 - i];
                self.data[j][n - 1 - i] = temp;
            }
        }
    }

    fn top_forward(&self) -> String {
        return self.data.first().unwrap().iter().collect();
    }

    fn bottom_forward(&self) -> String {
        return self.data.last().unwrap().iter().collect();
    }

    fn left_forward(&self) -> String {
        return self.data.iter().map(|v| v.first().unwrap()).collect();
    }

    fn right_forward(&self) -> String {
        return self.data.iter().map(|v| v.last().unwrap()).collect();
    }

    fn side_forward(&self, side: usize) -> String {
        match side {
            0 => self.top_forward(),
            1 => self.right_forward(),
            2 => self.bottom_forward(),
            3 => self.left_forward(),
            _ => unreachable!("Invalid side index given!"),
        }
    }
}

fn inverse(side: usize) -> usize {
    match side {
        0 => 2,
        1 => 3,
        2 => 0,
        3 => 1,
        _ => unreachable!("Invalid side index given!"),
    }
}

fn read_tile(lines: &mut Box<dyn Iterator<Item = String>>) -> Option<Tile> {
    if let Some(header) = lines.next() {
        let id: u32 = header[5..header.len() - 1].parse().expect(&format!(
            "Failed to get tile id from tile header: {}",
            header,
        ));
        let mut data: Vec<Vec<char>> = vec![];
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            data.push(line.chars().collect());
        }
        if data.is_empty() || data[0].is_empty() || data.len() != data[0].len() {
            panic!("tile with id {} is non square!", id);
        }
        return Some(Tile::new(id, data));
    }
    return None;
}

fn resolve_neighbor(
    u_tile: &mut Tile,
    u_side: usize,
    tile_map: &mut HashMap<u32, Tile>,
    q: &mut VecDeque<u32>,
) {
    if u_tile.neighbors[u_side].is_none() {
        u_tile.neighbors[u_side] = Some(None);
        let v_side = inverse(u_side);
        let u_edge = u_tile.side_forward(u_side);
        'outer: for v_tile in tile_map.values_mut() {
            if v_tile.resolved {
                if v_tile.side_forward(v_side) == u_edge {
                    u_tile.neighbors[u_side] = Some(Some(v_tile.id));
                    v_tile.neighbors[v_side] = Some(Some(u_tile.id));
                    break 'outer;
                }
            } else {
                for _ in 0..2 {
                    for __ in 0..4 {
                        if v_tile.side_forward(v_side) == u_edge {
                            u_tile.neighbors[u_side] = Some(Some(v_tile.id));
                            v_tile.neighbors[v_side] = Some(Some(u_tile.id));
                            break 'outer;
                        }
                        v_tile.rotate_clockwise();
                    }
                    v_tile.flip_horizontally();
                }
            }
        }
        if let Some(Some(v_id)) = u_tile.neighbors[u_side].as_ref() {
            if !tile_map.get(v_id).unwrap().resolved {
                q.push_back(*v_id);
            }
        }
    }
}

fn make_stitched_tile(tile_map: &HashMap<u32, Tile>) -> Tile {
    let mut data: Vec<Vec<char>> = vec![];
    let mut col_id = None;
    for tile in tile_map.values() {
        if tile.resolved
            && tile.neighbors[0].as_ref().unwrap().is_none()
            && tile.neighbors[3].as_ref().unwrap().is_none()
        {
            col_id = Some(tile.id);
            break;
        }
    }
    while let Some(cid) = col_id {
        let mut row_idx: usize = 0;
        let mut row_id = Some(cid);
        while let Some(rid) = row_id {
            let tile = tile_map.get(&rid).unwrap();
            for idx in 1..(tile.data.len() - 1) {
                while data.len() <= row_idx {
                    data.push(vec![]);
                }
                data[row_idx].extend(tile.data[idx][1..(tile.data[idx].len() - 1)].iter());
                row_idx += 1;
            }
            row_id = tile.neighbors[2].unwrap();
        }
        col_id = tile_map.get(&cid).unwrap().neighbors[1].unwrap();
    }
    return Tile::new(0, data);
}

fn stitch_tiles(tile_map: &mut HashMap<u32, Tile>) -> Tile {
    let mut q: VecDeque<u32> = VecDeque::new();
    let start_id = tile_map.values().next().unwrap().id;
    q.push_back(start_id);
    while !q.is_empty() {
        let u_id = q.pop_front().unwrap();
        let mut u_tile = tile_map.remove(&u_id).unwrap();
        if !u_tile.resolved {
            for i in 0..4 {
                resolve_neighbor(&mut u_tile, i, tile_map, &mut q);
            }
            u_tile.mark_resolved();
        }
        tile_map.insert(u_tile.id, u_tile);
    }
    return make_stitched_tile(tile_map);
}

fn count_monsters(stitched: &Tile, monster: &[[char; 20]; 3]) -> u64 {
    // should we count overlapping ones?
    let mut count = 0;
    for i in 0..(stitched.data.len() - 3 + 1) {
        for j in 0..(stitched.data[i].len() - 20 + 1) {
            let mut ok = true;
            'outer: for mi in 0..3 {
                for mj in 0..20 {
                    if monster[mi][mj] == '?' {
                        continue;
                    }
                    if stitched.data[i + mi][j + mj] != monster[mi][mj] {
                        ok = false;
                        break 'outer;
                    }
                }
            }
            if ok {
                count += 1;
            }
        }
    }
    return count;
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let mut tile_map: HashMap<u32, Tile> = HashMap::new();
    while let Some(tile) = read_tile(&mut lines) {
        tile_map.insert(tile.id, tile);
    }
    let mut stitched = stitch_tiles(&mut tile_map);
    let monster: [[char; 20]; 3] = [
        [
            '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?',
            '?', '#', '?',
        ],
        [
            '#', '?', '?', '?', '?', '#', '#', '?', '?', '?', '?', '#', '#', '?', '?', '?', '?',
            '#', '#', '#',
        ],
        [
            '?', '#', '?', '?', '#', '?', '?', '#', '?', '?', '#', '?', '?', '#', '?', '?', '#',
            '?', '?', '?',
        ],
    ];
    let mut hc: u64 = 0;
    for i in 0..stitched.data.len() {
        for j in 0..stitched.data[i].len() {
            if stitched.data[i][j] == '#' {
                hc += 1;
            }
        }
    }
    let mut answer: u64 = hc;
    for _ in 0..2 {
        for __ in 0..4 {
            answer = answer.min(hc - count_monsters(&stitched, &monster) * 15);
            stitched.rotate_clockwise();
        }
        stitched.flip_horizontally();
    }
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
        assert_eq!(solve(Box::new(it)), 273);
    }
}
