// https://leetcode.com/problems/non-overlapping-intervals/

use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() == 0 {
            return 0;
        }
        let mut s_intervals = intervals.clone();
        s_intervals.sort();
        let mut next_gte: Vec<usize> = vec![s_intervals.len(); s_intervals.len()];

        // O(N^2) !!
        for i in 0..s_intervals.len() {
            let mut ptr = i + 1;
            while ptr < s_intervals.len() && s_intervals[ptr][0] < s_intervals[i][1] {
                ptr += 1;
            }
            next_gte[i] = ptr;
        }

        let mut cache: Vec<Vec<i32>> = vec![vec![0, 1]; s_intervals.len() + 1];
        for i in (0..s_intervals.len() - 1).rev() {
            cache[i][0] = max(cache[i + 1][0], cache[i + 1][1]);
            cache[i][1] = 1;
            if next_gte[i] < s_intervals.len() {
                cache[i][1] += max(cache[next_gte[i]][0], cache[next_gte[i]][1])
            }
        }
        return s_intervals.len() as i32 - max(cache[0][0], cache[0][1]);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            2
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
            0
        );
    }
}
