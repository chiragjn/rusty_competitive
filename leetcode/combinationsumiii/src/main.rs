// https://leetcode.com/problems/combination-sum-iii/

struct Combinations<'a, T: Copy> {
    n: usize,
    r: usize,
    pool: &'a Vec<T>,
    state: Vec<usize>,
    _has_next: bool,
}

impl<'a, T: Copy> Combinations<'a, T> {
    fn new(sequence: &'a Vec<T>, r: usize) -> Self {
        return Self {
            n: sequence.len(),
            r: r,
            pool: sequence,
            state: (0..r as usize).collect(),
            _has_next: true,
        };
    }
}

impl<'a, T: Copy> Iterator for Combinations<'a, T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if !self._has_next {
            return None;
        }
        let item: Self::Item = self.state.iter().map(|&idx| self.pool[idx]).collect();
        self._has_next = false;
        for i in (0..self.r).rev() {
            if self.state[i] != i + self.n - self.r {
                self._has_next = true;
                self.state[i] += 1;
                for j in i + 1..self.r {
                    self.state[j] = self.state[j - 1] + 1;
                }
                break;
            }
        }
        return Some(item);
    }
}

struct Solution {}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        if k > 9 {
            return vec![];
        }
        let pool: Vec<i32> = (1..10).collect();
        let it = Combinations::new(&pool, k as usize);
        let mut answer: Vec<Vec<i32>> = vec![];
        for v in it {
            if v.iter().sum::<i32>() == n {
                answer.push(v);
            }
        }
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
        assert_eq!(
            Solution::combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
    }
}
