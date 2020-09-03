// https://leetcode.com/problems/repeated-substring-pattern/
//
// This is an overkill solution, just to practice KMP and some generics, lifetimes, trait impl
//
// Easier solution is to iter over prefixes of the string, if the total length is a mutliplier of
// prefix length then check rest of the string for repetition and return true as soon as that is met

use std::iter::Iterator;

struct KMP<'a, T: Eq> {
    needle: &'a [T],
    table: Vec<isize>,
}

struct KMPMatches<'a, T: Eq> {
    hay: &'a [T],
    needle: &'a [T],
    table: &'a Vec<isize>,
    hay_idx: usize,
    needle_idx: isize,
}

impl<'a, T: Eq> Iterator for KMPMatches<'a, T> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while self.hay_idx < self.hay.len() {
            if self.needle[self.needle_idx as usize] == self.hay[self.hay_idx] {
                self.hay_idx += 1;
                self.needle_idx += 1;
                if self.needle_idx as usize == self.needle.len() {
                    let m = self.hay_idx - self.needle_idx as usize;
                    self.needle_idx = self.table[self.needle_idx as usize];
                    return Some(m);
                }
            } else {
                self.needle_idx = self.table[self.needle_idx as usize];
                if self.needle_idx < 0 {
                    self.hay_idx += 1;
                    self.needle_idx += 1;
                }
            }
        }
        return None;
    }
}

impl<'a, T: Eq> KMP<'a, T> {
    fn new(needle: &'a [T]) -> Self {
        let mut table: Vec<isize> = vec![-1; needle.len() + 1];
        let n = needle.len();
        let mut pos: usize = 1;
        let mut cnd: isize = 0;
        while pos < n {
            if needle[pos] == needle[cnd as usize] {
                table[pos] = table[cnd as usize];
            } else {
                table[pos] = cnd;
                cnd = table[cnd as usize];
                while cnd >= 0 && needle[pos] != needle[cnd as usize] {
                    cnd = table[cnd as usize];
                }
            }
            pos += 1;
            cnd += 1;
        }
        table[pos] = cnd;
        return Self {
            needle: needle,
            table: table,
        };
    }

    fn matches(&'a self, hay: &'a [T]) -> KMPMatches<'a, T> {
        return KMPMatches {
            hay: hay,
            needle: &self.needle,
            table: &self.table,
            hay_idx: 0,
            needle_idx: 0,
        };
    }
}

struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let needle: Vec<char> = s.chars().collect();
        let hay: Vec<char> = s.chars().chain(s.chars()).collect();
        let finder = KMP::new(&needle[..]);
        let mut it = finder.matches(&hay[..]);
        it.next();
        if let Some(i) = it.next() {
            return i < s.len();
        }
        return false;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Solution, KMP};

    #[test]
    fn kmp_test() {
        let needle = &vec![1, 2, 1, 2];
        let kmp = KMP::new(&needle);
        assert_eq!(
            kmp.matches(&vec![3, 4, 5, 1, 2, 1, 2, 6, 7, 1, 2, 1, 2, 1, 2, 3])
                .collect::<Vec<usize>>(),
            vec![3, 9, 11]
        );
        assert_eq!(
            kmp.matches(&vec![3, 4, 5, 1, 2, 1, 3, 6, 7, 1, 4, 1, 5, 1, 2, 3])
                .collect::<Vec<usize>>(),
            vec![]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("abab")),
            true
        );
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("aba")),
            false
        );
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("abcabcabcabc")),
            true
        );
    }
}
