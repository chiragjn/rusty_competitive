// https://leetcode.com/problems/compare-version-numbers/

use std::cmp::max;
use std::iter::repeat;

struct Solution {}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1: Vec<u32> = version1
            .split('.')
            .map(|x_str| x_str.parse::<u32>().unwrap())
            .collect();
        let mut v2: Vec<u32> = version2
            .split('.')
            .map(|x_str| x_str.parse::<u32>().unwrap())
            .collect();
        let n = max(v1.len(), v2.len());
        v1.extend(repeat(0).take(n - v1.len()));
        v2.extend(repeat(0).take(n - v2.len()));
        if v1 == v2 {
            return 0;
        } else if v1 < v2 {
            return -1;
        } else {
            return 1;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::compare_version(String::from("0.1"), String::from("1.1")),
            -1
        );
        assert_eq!(
            Solution::compare_version(String::from("1.0.1"), String::from("1")),
            1
        );
        assert_eq!(
            Solution::compare_version(String::from("7.5.2.4"), String::from("7.5.3")),
            -1
        );
        assert_eq!(
            Solution::compare_version(String::from("1.01"), String::from("1.001")),
            0
        );
        assert_eq!(
            Solution::compare_version(String::from("1.0"), String::from("1.0.0")),
            0
        );
    }
}
