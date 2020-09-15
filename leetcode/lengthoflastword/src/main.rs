// https://leetcode.com/problems/length-of-last-word/

struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        return s.trim().chars().rev().take_while(|&c| c != ' ').count() as i32;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_last_word(String::from("abcd   ")), 4);
    }
}
