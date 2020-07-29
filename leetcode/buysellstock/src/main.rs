// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/

use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cache = vec![vec![0; 2]; prices.len() + 3];
        for i in (0..prices.len()).rev() {
            cache[i][0] = max(-prices[i] + cache[i + 1][1], cache[i + 1][0]);
            cache[i][1] = max(prices[i] + cache[i + 2][0], cache[i + 1][1]);
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
        assert_eq!(Solution::max_profit(vec![]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }
}
