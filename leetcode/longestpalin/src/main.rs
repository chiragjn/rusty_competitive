// https://leetcode.com/problems/longest-palindrome/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counter: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        let mut answer: i32 = 0;
        let mut odd_p: bool = false;
        for val in counter.values() {
            answer += val / 2;
            odd_p |= val % 2 == 1;
        }
        return (answer * 2) + (odd_p as i32);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_palindrome(String::from("abccccdd")), 7);
        assert_eq!(Solution::longest_palindrome(String::from("a")), 1);
        assert_eq!(Solution::longest_palindrome(String::from("abcd")), 1);
        assert_eq!(
            Solution::longest_palindrome(String::from("aaabbbcccddd")),
            9
        );
        assert_eq!(Solution::longest_palindrome(String::from("bbccccdd")), 8);
    }
}
