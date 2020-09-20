// https://leetcode.com/problems/unique-paths-iii/
// make this code better

struct Solution {}

fn recurse(
    i: isize,
    j: isize,
    len: i32,
    grid: &Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    end: &(usize, usize),
    path_len: i32,
) -> i32 {
    if i < 0 || j < 0 {
        return 0;
    }
    let iu = i as usize;
    let ju = j as usize;
    if iu >= grid.len() || ju >= grid[iu].len() || visited[iu][ju] {
        return 0;
    }
    if (iu, ju) == *end {
        if len == path_len {
            return 1;
        }
        return 0;
    }
    visited[iu][ju] = true;
    let mut answer = 0;
    answer += recurse(i + 1, j, len + 1, grid, visited, end, path_len);
    answer += recurse(i - 1, j, len + 1, grid, visited, end, path_len);
    answer += recurse(i, j + 1, len + 1, grid, visited, end, path_len);
    answer += recurse(i, j - 1, len + 1, grid, visited, end, path_len);
    visited[iu][ju] = false;
    return answer;
}

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;
        let mut visited: Vec<Vec<bool>> = vec![];
        let mut path_len = 0;
        for i in 0..grid.len() {
            visited.push(vec![]);
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    start = Some((i, j));
                }
                if grid[i][j] == 2 {
                    end = Some((i, j));
                }
                if grid[i][j] == -1 {
                    visited[i].push(true);
                } else {
                    path_len += 1;
                    visited[i].push(false);
                }
            }
        }
        if start.is_none() || end.is_none() {
            return 0;
        }
        let start = start.unwrap();
        let end = end.unwrap();
        return recurse(
            start.0 as isize,
            start.1 as isize,
            1,
            &grid,
            &mut visited,
            &end,
            path_len,
        );
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
            2
        );

        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
            4
        );

        assert_eq!(Solution::unique_paths_iii(vec![vec![0, 1], vec![2, 0]]), 0);
    }
}
