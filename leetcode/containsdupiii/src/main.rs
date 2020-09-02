// https://leetcode.com/problems/contains-duplicate-iii/

use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if k <= 0 || t < 0 {
            return false;
        }
        let mut set: BTreeSet<i32> = BTreeSet::new();
        let k = k as usize;
        let t = t as i64;
        for (i, num) in nums.iter().enumerate() {
            if let Some(x) = set.range(..=num).next_back() {
                if (*num as i64 - *x as i64).abs() <= t {
                    return true;
                }
            }
            if let Some(x) = set.range(num..).next() {
                if (*num as i64 - *x as i64).abs() <= t {
                    return true;
                }
            }
            set.insert(*num);
            if i >= k {
                set.remove(&nums[i - k]);
            }
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
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![1, 2, 3, 1],
            3,
            0
        ));
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![1, 0, 1, 1],
            1,
            2
        ));
        assert!(!Solution::contains_nearby_almost_duplicate(
            vec![1, 5, 9, 1, 5, 9],
            2,
            3
        ));
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![1, 5, 9, 1, 5, 9],
            3,
            3
        ));
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![1, 4, 7, 10, 13, 1],
            5,
            2
        ));
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![1, 4, 7, 10, 13, 10, 1],
            5,
            2
        ));
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![4, 1, 7, 10, 13, 10, 1],
            5,
            1
        ));
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![0, 2147483647],
            1,
            2147483647
        ));
        assert!(!Solution::contains_nearby_almost_duplicate(
            vec![-1, 2147483647],
            1,
            2147483647
        ));
    }
}
