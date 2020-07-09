// https://leetcode.com/problems/island-perimeter/

struct Solution {}

impl Solution {
    fn recur(
        i: isize,
        j: isize,
        count_grid: &mut Vec<Vec<i32>>,
        grid: &Vec<Vec<i32>>,
        vis: &mut Vec<Vec<bool>>,
    ) {
        // println!("{:?}", (i, j));
        if i < 0
            || i >= grid.len() as isize
            || j < 0
            || j >= grid[0].len() as isize
            || grid[i as usize][j as usize] == 0
        {
            count_grid[(i + 1) as usize][(j + 1) as usize] += 1;
            return;
        }

        if vis[i as usize][j as usize] {
            return;
        }
        vis[i as usize][j as usize] = true;
        Solution::recur(i - 1, j, count_grid, grid, vis);
        Solution::recur(i + 1, j, count_grid, grid, vis);
        Solution::recur(i, j - 1, count_grid, grid, vis);
        Solution::recur(i, j + 1, count_grid, grid, vis);
        return;
    }

    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut count_grid = vec![vec![0i32; m + 2]; n + 2];
        let mut vis = vec![vec![false; m]; n];
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    Solution::recur(i as isize, j as isize, &mut count_grid, &grid, &mut vis);
                    break;
                }
            }
        }
        let answer: i32 = count_grid.iter().flatten().sum::<i32>();
        return answer;
    }
}

fn main() {
    let grid = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    println!("{:?}", Solution::island_perimeter(grid));
}
