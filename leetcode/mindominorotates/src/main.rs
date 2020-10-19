// https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/

use std::cmp::min;

struct Solution {}

impl Solution {
    fn _min_rotations_for(x: i32, a: &Vec<i32>, b: &Vec<i32>) -> Option<i32> {
        if a.iter().zip(b.iter()).all(|(&i, &j)| i == x || j == x) {
            return Some(min(
                a.iter().filter(|&&i| i != x).count(),
                b.iter().filter(|&&j| j != x).count(),
            ) as i32);
        }
        return None;
    }
    pub fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let best = (1..7)
            .map(|i| Solution::_min_rotations_for(i, &a, &b))
            .flatten()
            .min();
        return best.unwrap_or(-1);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]),
            2
        );
        assert_eq!(
            Solution::min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]),
            -1
        );
    }
}
