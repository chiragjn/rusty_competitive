// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let (mut low, mut high): (usize, usize) = (0, nums.len() - 1);
        let mut mid: usize;
        while high - low > 1 {
            mid = low + ((high - low) / 2);
            if nums[low] < nums[mid] && nums[mid] < nums[high] {
                high = mid;
            } else if nums[low] < nums[mid] && nums[mid] > nums[high] {
                low = mid;
            } else if nums[low] > nums[mid] && nums[mid] < nums[high] {
                high = mid;
            } else {
                unreachable!();
            }
        }
        return min(nums[low], nums[high]);
    }
}

fn main() {
    assert_eq!(Solution::find_min(vec![1]), 1);
    assert_eq!(Solution::find_min(vec![1, 2, 3]), 1);
    assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(Solution::find_min(vec![7, 0, 1, 2, 4, 5, 6]), 0);
}
