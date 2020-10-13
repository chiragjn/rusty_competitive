// https://leetcode.com/problems/buddy-strings/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() == b.len() {
            let mut a_diff = vec![];
            let mut b_diff = vec![];
            let mut a_counts: HashMap<char, u32> = HashMap::new();
            for (ac, bc) in a.chars().zip(b.chars()) {
                *a_counts.entry(ac).or_default() += 1;
                if ac != bc {
                    a_diff.push(ac);
                    b_diff.push(bc);
                }
            }
            if a_diff.len() == 0 && *a_counts.values().into_iter().max().unwrap_or(&0) >= 2 {
                return true;
            } else if a_diff.len() == 2 && a_diff[0] == b_diff[1] && a_diff[1] == b_diff[0] {
                return true;
            }
            return false;
        }
        return false;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::buddy_strings(String::from("ab"), String::from("ba")),
            true
        );
        assert_eq!(
            Solution::buddy_strings(String::from("ab"), String::from("ab")),
            false
        );
        assert_eq!(
            Solution::buddy_strings(String::from("aa"), String::from("aa")),
            true
        );
        assert_eq!(
            Solution::buddy_strings(String::from("aaaaaaabc"), String::from("aaaaaaacb")),
            true
        );
        assert_eq!(
            Solution::buddy_strings(String::from(""), String::from("aa")),
            false
        );
    }
}
