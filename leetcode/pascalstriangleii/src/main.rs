// https://leetcode.com/problems/pascals-triangle-ii/

use std::mem::swap;

struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        // 0 <= row_index <= 33
        let mut prev = vec![];
        let mut row = vec![];
        for i in 0..row_index + 1 {
            row.resize(i as usize + 1, 1);
            for j in 1..i as usize {
                row[j] = prev[j - 1] + prev[j];
            }
            swap(&mut row, &mut prev);
        }
        return prev;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }
}
