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

struct Cell {
    number: u64,
    marked: bool,
}

fn _mark(board: &mut Vec<Vec<Cell>>, draw: u64) -> Option<u64> {
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            if cell.number == draw {
                cell.marked = true;
            }
        }
    }
    let mut won: bool = false;
    for row in board.iter() {
        won |= row.iter().all(|c| c.marked);
    }
    for j in 0..board.iter().next().map(|v| v.len()).unwrap_or(0) {
        let mut col_won: bool = true;
        for row in board.iter() {
            col_won &= row[j].marked;
        }
        won |= col_won;
    }
    if won {
        let mut sum: u64 = 0;
        for row in board.iter() {
            for cell in row.iter() {
                if !cell.marked {
                    sum += cell.number;
                }
            }
        }
        return Some(sum);
    }
    None
}

fn _simulate_board(
    lines: &mut Box<dyn Iterator<Item = String>>,
    draws: &[u64],
) -> Option<(usize, u64)> {
    let mut board: Vec<Vec<Cell>> = vec![];
    for _ in 0..5 {
        board.push(
            lines
                .next()
                .expect("Failed to read line for board")
                .split_whitespace()
                .into_iter()
                .map(|x| Cell {
                    number: x.parse().expect("Failed to parse as u64: {x}"),
                    marked: false,
                })
                .collect(),
        );
    }
    for (i, &draw) in draws.iter().enumerate() {
        if let Some(unmarked_sum) = _mark(&mut board, draw) {
            return Some((i, draw * unmarked_sum));
        }
    }
    None
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let draws: Vec<u64> = lines
        .next()
        .expect("Failed to read bingo draws on first line")
        .split(',')
        .into_iter()
        .map(|x| x.parse().expect("Failed to parse as int"))
        .collect();
    lines.next();
    let mut wins: Vec<(usize, u64)> = vec![];
    loop {
        if let Some(win) = _simulate_board(&mut lines, &draws) {
            wins.push(win);
        };
        if lines.next().is_none() {
            break;
        }
    }
    let &(_, answer) = wins.iter().min().unwrap();
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
        let test_input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 4512);
    }
}
