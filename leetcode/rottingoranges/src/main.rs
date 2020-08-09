// https://leetcode.com/problems/rotting-oranges/

use std::cmp::max;
use std::collections::{HashSet, VecDeque};

struct Solution {}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let coords: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();
        let mut rotten: HashSet<(usize, usize)> = HashSet::new();
        let mut answer: i32 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 2 {
                    rotten.insert((i, j));
                    q.push_back((i, j, 0));
                }
            }
        }
        while !q.is_empty() {
            let (x, y, t) = q.pop_front().unwrap();
            answer = max(answer, t);
            for (a, b) in coords.iter() {
                let nx = x as isize + a;
                let ny = y as isize + b;
                if nx >= 0 && ny >= 0 {
                    let nxu = nx as usize;
                    let nyu = ny as usize;
                    if nxu < grid.len()
                        && nyu < grid[nxu].len()
                        && grid[nxu][nyu] == 1
                        && !rotten.contains(&(nxu, nyu))
                    {
                        rotten.insert((nxu, nyu));
                        q.push_back((nxu, nyu, t + 1));
                    }
                }
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != 0 && !rotten.contains(&(i, j)) {
                    return -1;
                }
            }
        }
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        );
    }
}
