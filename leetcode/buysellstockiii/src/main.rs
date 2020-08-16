// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/

use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }
        let n = prices.len();
        let mut cache = vec![vec![0; 5]; n + 1];
        for i in (0..n).rev() {
            for j in 0..4 {
                let sig = {
                    if j % 2 == 0 {
                        -1
                    } else {
                        1
                    }
                };
                cache[i][j] = max(cache[i + 1][j], (sig * prices[i]) + cache[i + 1][j + 1]);
            }
        }
        return cache[0][0];
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
