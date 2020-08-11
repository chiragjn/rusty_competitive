// https://leetcode.com/problems/h-index/

use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut h: i32 = 0;
        let (mut low, mut high, mut mid): (i32, i32, i32) = (0, citations.len() as i32, 0);
        while low <= high {
            mid = low + ((high - low) / 2);
            let count = citations.iter().fold(0, |acc, &x| acc + (x >= mid) as i32);
            if count >= mid {
                h = max(h, mid);
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        return h;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::h_index(vec![]), 0);
        assert_eq!(Solution::h_index(vec![10, 100, 1000]), 3);
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }
}
