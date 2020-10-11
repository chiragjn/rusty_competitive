// https://leetcode.com/problems/remove-duplicate-letters/

use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut last_occurence: HashMap<char, usize> = HashMap::new();
        let mut visited: HashSet<char> = HashSet::new();
        let mut stack: Vec<char> = vec!['!'];
        for (i, c) in s.chars().enumerate() {
            *last_occurence.entry(c).or_default() = i;
        }

        for (i, c) in s.chars().enumerate() {
            if !visited.contains(&c) {
                while c < *stack.last().unwrap() && last_occurence[stack.last().unwrap()] > i {
                    visited.remove(&stack.pop().unwrap());
                }
                stack.push(c);
                visited.insert(c);
            }
        }
        let mut answer: String = String::new();
        answer.extend(stack.iter().skip(1));
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::remove_duplicate_letters(String::from("bcabc")),
            String::from("abc")
        );
        assert_eq!(
            Solution::remove_duplicate_letters(String::from("cbacdcbc")),
            String::from("acdb")
        );
    }
}
