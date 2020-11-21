// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        let (mut low, mut high): (usize, usize) = (0, nums.len() - 1);
        let mut mid: usize;
        while low <= high {
            mid = low + ((high - low) / 2);
            if nums[mid] == target {
                return true;
            }

            if nums[mid] == nums[low] {
                // we can't partition reliably
                low += 1;
                continue;
            }

            if nums[low] <= nums[mid] && nums[low] > target {
                low = mid + 1;
            } else if nums[low] > nums[mid] && nums[low] <= target {
                high = mid - 1;
            } else if (nums[low] <= nums[mid] && nums[low] <= target)
                || (nums[low] > nums[mid] && nums[low] > target)
            {
                if nums[mid] < target {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        }
        return false;
    }
}

fn main() {
    assert_eq!(Solution::search(vec![1], 1), true);
    assert_eq!(Solution::search(vec![1, 1, 1], 1), true);
    assert_eq!(
        Solution::search(vec![1, 1, 1, 2, 3, 4, 5, 1, 1, 1], 6),
        false
    );
    assert_eq!(
        Solution::search(vec![1, 1, 1, 2, 3, 4, 5, 1, 1, 1], 2),
        true
    );
    assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
}
