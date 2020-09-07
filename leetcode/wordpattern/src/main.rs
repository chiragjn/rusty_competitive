// https://leetcode.com/problems/word-pattern/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, string: String) -> bool {
        let mut piter = pattern.chars();
        let mut siter = string.split_whitespace();
        let mut map: HashMap<char, &str> = HashMap::new();
        let mut rev_map: HashMap<&str, char> = HashMap::new();
        loop {
            match (piter.next(), siter.next()) {
                (Some(c), Some(word)) => match (map.get(&c), rev_map.get(word)) {
                    (None, None) => {
                        map.insert(c, word);
                        rev_map.insert(word, c);
                    }
                    (Some(mapped_word), Some(mapped_c)) => {
                        if *mapped_c != c || *mapped_word != word {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                },
                (None, None) => {
                    break;
                }
                _ => {
                    return false;
                }
            }
        }
        return true;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat dog")),
            true
        );
        assert_eq!(
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat fish")),
            false
        );
        assert_eq!(
            Solution::word_pattern(String::from("aaaa"), String::from("dog cat cat dog")),
            false
        );
        assert_eq!(
            Solution::word_pattern(String::from("abba"), String::from("dog dog dog dog")),
            false
        );
        assert_eq!(
            Solution::word_pattern(String::from("aaa"), String::from("aa aa aa aa")),
            false
        );
    }
}
