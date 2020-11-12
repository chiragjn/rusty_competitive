// https://leetcode.com/problems/permutations-ii/

use std::cmp::Ord;
use std::iter::FromIterator;

struct NextPermutationsGenerator<T> {
    data: Vec<T>,
    has_next: bool,
}

impl<T> NextPermutationsGenerator<T> {
    fn new(data: Vec<T>) -> Self {
        return NextPermutationsGenerator {
            data: data,
            has_next: true,
        };
    }
}

impl<T: Clone + Copy + Ord> Iterator for NextPermutationsGenerator<T> {
    // This implementation is taken from C++'s std::next_permutation except this returns a copy per
    // permutation. Code is a bit gnarly because of iterator state management and special cases
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.has_next {
            match self.data.len() {
                0 => {
                    return None;
                }
                1 => {
                    self.has_next = false;
                    return Some(self.data.clone());
                }
                _ => {
                    let ret = self.data.clone();
                    let left: usize = 0;
                    let right: usize = self.data.len();
                    let mut i = right - 1;
                    loop {
                        let i_copy = i;
                        i -= 1;
                        if self.data[i] < self.data[i_copy] {
                            let mut j = right - 1;
                            while !(self.data[i] < self.data[j]) {
                                j -= 1;
                            }
                            self.data.swap(i, j);
                            self.data[i_copy..right].reverse();
                            return Some(ret);
                        }
                        if i == left {
                            self.data[left..right].reverse();
                            self.has_next = false;
                            return Some(ret);
                        }
                    }
                }
            }
        }
        return None;
    }
}

struct Solution {}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
        let perms: Vec<Vec<i32>> = Vec::from_iter(NextPermutationsGenerator::new(nums));
        return perms;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
        assert_eq!(
            Solution::permute_unique(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
