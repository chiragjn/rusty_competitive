// https://leetcode.com/problems/minimum-cost-to-move-chips-to-the-same-position/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut pos2chips: HashMap<usize, i32> = HashMap::new();
        for &pos in position.iter() {
            *pos2chips.entry(pos as usize).or_default() += 1;
        }
        let mut sums: Vec<i32> = vec![0, 0];
        for (pos, chips) in pos2chips.iter() {
            sums[pos % 2] += chips;
        }
        return *sums.iter().min().unwrap_or(&0); // this can panic!
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2, 3]), 1);
        assert_eq!(Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
        assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2, 3]), 1);
    }
}
