// https://leetcode.com/problems/teemo-attacking/

use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        if time_series.is_empty() {
            return 0;
        }
        let mut poisoned = 0;
        for i in 0..time_series.len() - 1 {
            poisoned += min(duration, time_series[i + 1] - time_series[i]);
        }
        poisoned += duration;
        return poisoned;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
        assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
        assert_eq!(Solution::find_poisoned_duration(vec![1, 2, 3, 4], 1), 4);
        assert_eq!(Solution::find_poisoned_duration(vec![10], 2), 2);
        assert_eq!(
            Solution::find_poisoned_duration(vec![1, 8, 100, 105, 110], 10),
            37
        );
    }
}
