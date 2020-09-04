// https://leetcode.com/problems/partition-labels/

use std::cmp::max;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let chars: Vec<char> = s.chars().collect();
        let mut last: HashMap<char, usize> = HashMap::new();
        for (i, c) in chars.iter().enumerate().rev() {
            last.entry(*c).or_insert(i);
        }
        let cmax: Vec<usize> = chars
            .iter()
            .scan(0usize, |mx, c| {
                *mx = max(*mx, last[c]);
                Some(*mx)
            })
            .collect();
        let mut sizes: Vec<i32> = vec![];
        let mut start: usize = 0;
        let mut idx: usize = 0;

        while idx < chars.len() {
            if cmax[cmax[idx]] <= cmax[idx] {
                idx = cmax[idx] + 1;
                sizes.push((idx - start) as i32);
                start = idx;
            } else {
                idx += 1;
            }
        }

        return sizes;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::partition_labels(String::from("ababcbacadefegdehijhklij")),
            vec![9, 7, 8]
        );
        assert_eq!(
            Solution::partition_labels(String::from("abcd")),
            vec![1, 1, 1, 1]
        );
        assert_eq!(Solution::partition_labels(String::from("abcda")), vec![5]);
        assert_eq!(
            Solution::partition_labels(String::from("abacbdef")),
            vec![5, 1, 1, 1]
        );
    }
}
