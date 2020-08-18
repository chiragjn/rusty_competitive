// https://leetcode.com/problems/numbers-with-same-consecutive-differences/

use std::collections::HashSet;

fn recur(running: i32, pos: i32, n: &i32, k: &i32, buffer: &mut HashSet<i32>) {
    if pos == *n {
        buffer.insert(running);
        return;
    }
    let last = running % 10;
    if last + k >= 0 && last + k <= 9 {
        recur(running * 10 + (last + k), pos + 1, n, k, buffer);
    }
    if last - k >= 0 && last - k <= 9 {
        recur(running * 10 + (last - k), pos + 1, n, k, buffer);
    }
}

struct Solution {}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut answer: HashSet<i32> = HashSet::new();
        let mut low: i32 = 0;
        if n > 1 {
            low += 1;
        }
        for i in low..10 {
            recur(i, 1, &n, &k, &mut answer);
        }
        return answer.into_iter().collect();
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{HashSet, Solution};
    use std::iter::FromIterator;

    #[test]
    fn test() {
        assert_eq!(
            HashSet::<i32>::from_iter(Solution::nums_same_consec_diff(3, 7)),
            HashSet::<i32>::from_iter(vec![181, 292, 707, 818, 929])
        );
        assert_eq!(
            HashSet::<i32>::from_iter(Solution::nums_same_consec_diff(2, 1)),
            HashSet::<i32>::from_iter(vec![
                10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98
            ])
        );
        assert_eq!(
            HashSet::<i32>::from_iter(Solution::nums_same_consec_diff(1, 0)),
            HashSet::<i32>::from_iter(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        );
        assert_eq!(
            HashSet::<i32>::from_iter(Solution::nums_same_consec_diff(1, 1)),
            HashSet::<i32>::from_iter(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        );
        assert_eq!(
            HashSet::<i32>::from_iter(Solution::nums_same_consec_diff(2, 0)),
            HashSet::<i32>::from_iter(vec![11, 22, 33, 44, 55, 66, 77, 88, 99])
        );
    }
}
