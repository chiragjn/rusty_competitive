// https://leetcode.com/problems/subarray-product-less-than-k/

struct Solution {}

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        /* 0 < nums.length <= 50000
        0 < nums[i] < 1000
        0 <= k < 10^6 */
        let mut answer: usize = 0;
        if nums.is_empty() {
            return answer as i32;
        }
        let k64 = k as i64;
        let mut p: i64 = 1;
        let mut start: usize = 0;
        for end in 0..nums.len() {
            p *= nums[end] as i64;
            while start < end && p >= k64 {
                p /= nums[start] as i64;
                start += 1;
            }
            if p < k64 {
                answer += end - start + 1;
            }
        }
        return answer as i32;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 7),
            3
        );
        // Regression
        assert_eq!(
            Solution::num_subarray_product_less_than_k(
                vec![10, 9, 10, 4, 3, 8, 3, 3, 6, 2, 10, 10, 9, 3],
                19
            ),
            18
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(
                vec![10, 3, 3, 7, 2, 9, 7, 4, 7, 2, 8, 6, 5, 1, 5],
                30
            ),
            26
        )
    }
}
