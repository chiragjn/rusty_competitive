// https://leetcode.com/submissions/detail/358959713/

use std::cmp;

struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut answers = vec![1000000000i32; (n + 1) as usize];
        let squares: Vec<usize> = (0..501).map(|x| x * x).collect();
        answers[0] = 0;
        for i in 1 as usize..=n as usize {
            for j in &squares {
                if j <= &i {
                    answers[i] = cmp::min(answers[i], 1 + answers[i - j]);
                } else {
                    break;
                }
            }
        }
        return answers[n as usize];
    }
}

pub fn main() {
    println!("{:?}", Solution::num_squares(100));
}
