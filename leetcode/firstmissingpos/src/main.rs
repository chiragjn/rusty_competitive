// https://leetcode.com/problems/first-missing-positive/

struct Solution {}

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        // move as mut because we want O(1) extra space solution
        let n = nums.len() as i32;
        let mx = n + 2;
        for num in nums.iter_mut() {
            if *num <= 0 {
                *num = mx;
            }
        }
        for i in 0..nums.len() {
            let k = nums[i].abs();
            if k >= 1 && k <= n {
                nums[(k - 1) as usize] = 0 - (nums[(k - 1) as usize].abs());
            }
        }
        for i in 1..nums.len() + 1 {
            if nums[i - 1] > 0 {
                return i as i32;
            }
        }
        return (nums.len() + 1) as i32;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![0, 0, 0]), 1);
        assert_eq!(Solution::first_missing_positive(vec![-1, -2, -1000]), 1);
        assert_eq!(Solution::first_missing_positive(vec![1, 3, 2]), 4);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
