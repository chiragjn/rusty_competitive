// https://leetcode.com/problems/combination-sum/

use std::mem::swap;

struct Combinations {
    candidates: Vec<i32>,
    buffer: Vec<i32>,
    solutions: Vec<Vec<i32>>,
}

impl Combinations {
    fn new(candidates: &Vec<i32>) -> Self {
        let mut candidates = candidates.clone();
        candidates.sort();
        return Self {
            candidates: candidates,
            buffer: vec![],
            solutions: vec![],
        };
    }

    fn _recurse(&mut self, i: usize, target: i32) {
        if target == 0 {
            self.solutions.push(self.buffer.clone());
            return;
        }
        if target < 0 || i >= self.candidates.len() || self.candidates[i] > target {
            return;
        }
        let n = self.buffer.len();
        let mut t = target;
        while t > 0 {
            t -= self.candidates[i];
            self.buffer.push(self.candidates[i]);
            self._recurse(i + 1, t);
        }
        while self.buffer.len() > n {
            self.buffer.pop();
        }
        self._recurse(i + 1, target);
    }

    fn get(&mut self, target: i32) -> Vec<Vec<i32>> {
        self._recurse(0, target);
        assert!(self.buffer.is_empty());
        let mut answer: Vec<Vec<i32>> = vec![];
        swap(&mut self.solutions, &mut answer);
        answer.sort();
        return answer;
    }
}

struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combo = Combinations::new(&candidates);
        return combo.get(target);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2], 1),
            vec![] as Vec<Vec<i32>> // is there a better way to annotate ?
        );

        assert_eq!(Solution::combination_sum(vec![1], 1), vec![vec![1]]);

        assert_eq!(Solution::combination_sum(vec![1], 2), vec![vec![1, 1]]);
    }
}
