// https://leetcode.com/problems/house-robber-ii/

use std::cmp::max;

struct Solution {}

impl Solution {
    fn _rob(nums: &[i32]) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut cache: Vec<i32> = vec![0; nums.len() + 3];
        for i in (0..nums.len()).rev() {
            cache[i] = max(cache[i + 1], nums[i] + cache[i + 2]);
        }
        return cache[0];
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        return max(
            Solution::_rob(&nums[..nums.len() - 1]),
            Solution::_rob(&nums[1..]),
        );
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 11);
    }
}
