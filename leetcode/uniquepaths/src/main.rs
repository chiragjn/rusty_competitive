// https://leetcode.com/problems/unique-paths/

use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let N: i64 = (m - 1 + n - 1).into();
        let R: i64 = (m - 1).into();
        let num_start = max(R, N - R) + 1;
        let den_till = min(R, N - R);
        let mut answer: i64 = 1;
        // println!("{:?}", (N, R, num_start, den_till));
        for i in num_start..=N {
            answer *= i;
        }
        for i in 2..=den_till {
            answer /= i;
        }
        return answer as i32;
    }
}

pub fn main() {
    println!("{:?}", Solution::unique_paths(3, 10));
}
