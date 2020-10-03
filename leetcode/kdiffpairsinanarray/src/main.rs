// https://leetcode.com/problems/k-diff-pairs-in-an-array/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts: HashMap<i32, u32> = HashMap::new();
        for &num in nums.iter() {
            *counts.entry(num).or_default() += 1;
        }
        let mut answer = 0;
        for &num in counts.keys() {
            let other = num - k;
            let required_count = {
                if num == other {
                    2
                } else {
                    1
                }
            };
            if *counts.get(&other).unwrap_or(&0) >= required_count {
                answer += 1;
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
        assert_eq!(Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
        assert_eq!(Solution::find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
        assert_eq!(Solution::find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
        assert_eq!(
            Solution::find_pairs(vec![1, 2, 4, 4, 3, 3, 0, 9, 2, 3], 3),
            2
        );
        assert_eq!(Solution::find_pairs(vec![-1, -2, -3], 1), 2);
    }
}
