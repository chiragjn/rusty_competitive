// https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/

use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut low = 1;
        let mut high = *nums.iter().max().unwrap_or(&low);
        let mut answer = high;
        while low <= high {
            let mid = low + ((high - low) / 2);
            let sum: i32 = nums
                .iter()
                .map(|&num| (num as f32 / mid as f32).ceil() as i32)
                .sum();
            if sum > threshold {
                low = mid + 1;
            } else {
                answer = min(answer, mid);
                high = mid - 1;
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
        assert_eq!(Solution::smallest_divisor(vec![1, 2, 5, 9], 6), 5);
        assert_eq!(Solution::smallest_divisor(vec![2, 3, 5, 7, 11], 11), 3);
        assert_eq!(Solution::smallest_divisor(vec![19], 5), 4);
    }
}
