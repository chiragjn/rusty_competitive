// https://leetcode.com/problems/find-the-difference/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut scounter: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *scounter.entry(c).or_default() += 1;
        }
        let mut tcounter: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            *tcounter.entry(c).or_default() += 1;
        }
        for (&c, &count) in tcounter.iter() {
            if *scounter.entry(c).or_default() < count {
                return c;
            }
        }
        unreachable!("Invalid Input!");
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_the_difference(String::from("abcd"), String::from("abcde")),
            'e'
        );
        assert_eq!(
            Solution::find_the_difference(String::from("abcde"), String::from("abecde")),
            'e'
        );
        assert_eq!(
            Solution::find_the_difference(String::from(""), String::from("e")),
            'e'
        );
    }
}
