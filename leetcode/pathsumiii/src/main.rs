// https://leetcode.com/problems/path-sum-iii/
// O(N^2) solution, O(N) possible with some memoization at each node

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    /// Convenience method to construct Tree from leetcode input format
    fn from_vec(serialized: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        if serialized.len() == 0 || serialized[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(Self::new(serialized[0].unwrap())));
        let mut q: VecDeque<Rc<RefCell<Self>>> = VecDeque::new();
        q.push_back(root.clone());
        let mut idx: usize = 1;
        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            if idx < serialized.len() {
                if let Some(v) = serialized[idx] {
                    let left = Rc::new(RefCell::new(Self::new(v)));
                    node.borrow_mut().left = Some(left.clone());
                    q.push_back(left);
                }
                idx += 1;

                if idx < serialized.len() {
                    if let Some(v) = serialized[idx] {
                        let right = Rc::new(RefCell::new(Self::new(v)));
                        node.borrow_mut().right = Some(right.clone());
                        q.push_back(right);
                    }
                    idx += 1;
                }
            }
        }

        return Some(root);
    }
}

struct Solution {}

impl Solution {
    fn _count_paths(
        node: Option<Rc<RefCell<TreeNode>>>,
        sum_so_far: i32,
        target: &i32,
        answer: &mut i32,
    ) {
        if let Some(n) = node {
            if n.borrow().val + sum_so_far == *target {
                *answer += 1;
            }
            Solution::_count_paths(
                n.borrow().left.clone(),
                n.borrow().val + sum_so_far,
                target,
                answer,
            );
            Solution::_count_paths(
                n.borrow().right.clone(),
                n.borrow().val + sum_so_far,
                target,
                answer,
            );
        }
    }

    fn _dfs(node: Option<Rc<RefCell<TreeNode>>>, target: &i32, answer: &mut i32) {
        if let Some(n) = node {
            Solution::_dfs(n.borrow().left.clone(), target, answer);
            Solution::_dfs(n.borrow().right.clone(), target, answer);
            Solution::_count_paths(Some(n), 0, target, answer);
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut answer = 0;
        Solution::_dfs(root, &sum, &mut answer);
        return answer;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn test() {
        assert_eq!(
            Solution::path_sum(
                TreeNode::from_vec(vec![
                    Some(10),
                    Some(5),
                    Some(-3),
                    Some(3),
                    Some(2),
                    None,
                    Some(11),
                    Some(3),
                    Some(-2),
                    None,
                    Some(1)
                ]),
                8
            ),
            3
        );
    }
}
