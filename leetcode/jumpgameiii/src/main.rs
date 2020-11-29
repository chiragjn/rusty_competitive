// https://leetcode.com/problems/jump-game-iii/

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let start = start as usize;
        if let Some(_) = arr.iter().position(|&x| x == 0) {
            let mut q: VecDeque<usize> = VecDeque::new();
            let mut visited = vec![false; arr.len()];
            visited[start] = true;
            q.push_back(start);
            while !q.is_empty() {
                let u = q.pop_front().unwrap();
                if arr[u] == 0 {
                    return true;
                }
                let left = u as i32 - arr[u];
                let right = u + arr[u] as usize;
                if left >= 0 && !visited[left as usize] {
                    visited[left as usize] = true;
                    q.push_back(left as usize);
                }
                if right < arr.len() && !visited[right] {
                    visited[right] = true;
                    q.push_back(right);
                }
            }
        }
        return false;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5), true);
        assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0), true);
        assert_eq!(Solution::can_reach(vec![3, 0, 2, 1, 2], 2), false);
    }
}
