// https://leetcode.com/problems/maximum-product-subarray/

struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut prev_max: i64 = nums[0] as i64;
        let mut prev_min: i64 = nums[0] as i64;
        let mut best: i64 = nums[0] as i64;
        for num in nums[1..].iter() {
            let n = *num as i64;
            let curr_max = *[prev_max * n, prev_min * n, n].iter().max().unwrap();
            let curr_min = *[prev_max * n, prev_min * n, n].iter().min().unwrap();
            best = *[best, curr_max, curr_min].iter().max().unwrap();
            prev_max = curr_max;
            prev_min = curr_min;
        }
        return best as i32;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
        assert_eq!(Solution::max_product(vec![-2, 0, -1, 2, -3, 4, -5]), 120);
        assert_eq!(Solution::max_product(vec![-2, -3, -1]), 6);
    }
}
