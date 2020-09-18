// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut mn = std::i32::MAX;
        let mut answer = 0;
        for price in prices.iter() {
            mn = min(mn, *price);
            answer = max(answer, *price - mn);
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
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0);
    }
}
