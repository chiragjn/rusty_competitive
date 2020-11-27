// https://leetcode.com/problems/partition-equal-subset-sum/

use std::mem::swap;

struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut nums = nums.clone();
        nums.sort();
        let s: i32 = nums.iter().sum();
        if s % 2 == 0 {
            let hs = s / 2;
            let mut prev = vec![false; (hs + 1) as usize];
            let mut current = vec![false; (hs + 1) as usize];
            let mut prev_r = &mut prev;
            let mut curr_r = &mut current;
            prev_r[0] = true;
            for &num in nums.iter() {
                for j in 0..prev_r.len() {
                    curr_r[j] = prev_r[j];
                }
                for j in 0..prev_r.len() {
                    if (j + num as usize) >= curr_r.len() {
                        break;
                    }
                    if prev_r[j] {
                        curr_r[j + num as usize] = true;
                    }
                }
                swap(&mut prev_r, &mut curr_r);
            }
            return *prev_r.last().unwrap();
        }
        return false;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
    }
}
