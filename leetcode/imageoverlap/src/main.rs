// https://leetcode.com/problems/image-overlap/

use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let mut a_pad = vec![vec![0; 3 * n]; 3 * n];
        for i in 0..n {
            for j in 0..n {
                a_pad[n + i][n + j] = a[i][j];
            }
        }
        let mut answer = 0;
        for offi in 1..2 * n {
            for offj in 1..2 * n {
                let mut c = 0;
                for i in 0..n {
                    for j in 0..n {
                        c += a_pad[offi + i][offj + j] * b[i][j];
                    }
                }
                answer = max(answer, c);
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
            Solution::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
            ),
            3
        );
    }
}
