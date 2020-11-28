// https://leetcode.com/problems/sliding-window-maximum/
use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut answer: Vec<i32> = vec![];
        for (i, &num) in nums.iter().enumerate() {
            while !deque.is_empty() && *deque.front().unwrap() as i32 <= (i as i32 - k) {
                deque.pop_front();
            }
            while !deque.is_empty() && nums[*deque.back().unwrap()] <= num {
                deque.pop_back();
            }
            deque.push_back(i);
            if i as i32 >= k - 1 {
                answer.push(nums[*deque.front().unwrap()]);
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
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
        assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);
        assert_eq!(Solution::max_sliding_window(vec![9, 11], 2), vec![11]);
        assert_eq!(Solution::max_sliding_window(vec![4, -2], 2), vec![4]);
    }
}
