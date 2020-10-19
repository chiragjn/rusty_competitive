//

use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        // TODO: think of better solutions
        let k = k as usize;
        if k == 0 || prices.is_empty() {
            return 0;
        }
        let mut answer = 0;
        if (2 * k) > prices.len() {
            for (i, j) in prices[1..].iter().zip(prices[..prices.len() - 1].iter()) {
                answer += max(0, i - j);
            }
        } else {
            let mut cache = vec![vec![vec![std::i32::MIN / 2; 2]; k + 1]; prices.len()];
            // ith day, j transactions (buy) done, 1/0 held or not
            cache[0][0][0] = 0;
            cache[0][1][1] = -prices[0];
            for i in 1..prices.len() {
                for j in 0..(k + 1) {
                    // continue not buying or sell whatever is held
                    cache[i][j][0] = max(cache[i - 1][j][0], cache[i - 1][j][1] + prices[i]);
                    if j > 0 {
                        // we can buy!
                        cache[i][j][1] =
                            max(cache[i - 1][j][1], cache[i - 1][j - 1][0] - prices[i]);
                    }
                }
            }
            answer = cache[prices.len() - 1][0..(k + 1)]
                .iter()
                .map(|v| v[0])
                .max()
                .unwrap();
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
        assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
        assert_eq!(Solution::max_profit(1000, vec![]), 0);
        assert_eq!(Solution::max_profit(0, vec![1, 10000, 1000000]), 0);
    }
}
