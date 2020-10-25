// https://leetcode.com/problems/bag-of-tokens/

use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, p: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        tokens.sort();
        tokens.push(p);
        let (mut left, mut right): (usize, usize) = (0, tokens.len() - 1);
        let mut power = 0;
        let mut points = 1;
        let mut best = 0;
        loop {
            if right <= left || points <= 0 {
                break;
            }
            points -= 1;
            power += tokens[right];
            while left < right && power >= tokens[left as usize] {
                power -= tokens[left as usize];
                points += 1;
                left += 1;
            }
            right -= 1;
            best = max(best, points);
        }
        return best;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100], 50), 0);
        assert_eq!(Solution::bag_of_tokens_score(vec![100, 200], 150), 1);
        assert_eq!(Solution::bag_of_tokens_score(vec![200, 200, 200], 200), 1);
        assert_eq!(Solution::bag_of_tokens_score(vec![200, 200], 200), 1);
        assert_eq!(
            Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200),
            2
        );
    }
}
