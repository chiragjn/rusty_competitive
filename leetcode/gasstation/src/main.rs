// https://leetcode.com/problems/gas-station/
// Overkill solution. O(1) extra memory solution exists. TODO: implement O(1) memory solution

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.is_empty() {
            return -1;
        }
        let mut graph: Vec<i32> = vec![0];
        let mut run = 0;
        for i in (0..gas.len()).chain(0..gas.len()) {
            run += gas[i] - cost[i];
            graph.push(run);
        }
        let mut deque: VecDeque<usize> = VecDeque::new();
        for i in 0..gas.len() + 1 {
            while !deque.is_empty() && graph[*deque.back().unwrap()] > graph[i] {
                deque.pop_back();
            }
            deque.push_back(i);
        }
        for (i, j) in (0..gas.len()).zip(gas.len() + 1..) {
            if (gas[i] - cost[i]) >= 0 && (graph[*deque.front().unwrap()] - graph[i]) >= 0 {
                return i as i32;
            }
            while !deque.is_empty() && *deque.front().unwrap() <= i {
                deque.pop_front();
            }
            while !deque.is_empty() && graph[*deque.back().unwrap()] > graph[j] {
                deque.pop_back();
            }
            deque.push_back(j);
        }
        return -1;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::can_complete_circuit(
                vec![11, 0, 0, 3, 0, 5, 0, 0, 0, 8],
                vec![0, 7, 5, 0, 8, 0, 2, 2, 2, 0]
            ),
            9
        );
        assert_eq!(Solution::can_complete_circuit(vec![10], vec![5]), 0);
        assert_eq!(Solution::can_complete_circuit(vec![5], vec![10]), -1);
        assert_eq!(
            Solution::can_complete_circuit(
                vec![0, 0, 0, 8, 11, 0, 0, 3, 0, 5],
                vec![2, 2, 2, 0, 0, 7, 5, 0, 8, 0]
            ),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }
}
