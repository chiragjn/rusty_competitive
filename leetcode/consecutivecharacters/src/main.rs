// https://leetcode.com/problems/consecutive-characters/

use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut best: i32 = 0;
        let mut running: i32 = 0;
        let mut prev: char = '?'; // okay because of constraints
        for c in s.chars() {
            if c != prev {
                running = 0;
            }
            running += 1;
            best = max(best, running);
            prev = c;
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
        assert_eq!(Solution::max_power(String::from("")), 0);
        assert_eq!(Solution::max_power(String::from("leetcode")), 2);
        assert_eq!(Solution::max_power(String::from("abbcccddddeeeeedcba")), 5);
        assert_eq!(Solution::max_power(String::from("triplepillooooow")), 5);
        assert_eq!(Solution::max_power(String::from("hooraaaaaaaaaaay")), 11);
        assert_eq!(Solution::max_power(String::from("tourist")), 1);
    }
}
