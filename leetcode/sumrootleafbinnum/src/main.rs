// https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/

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
    fn _traverse(node: &Option<Rc<RefCell<TreeNode>>>, accumulated: i32, result: &mut i32) {
        if let Some(u) = node {
            let mut accumulated = accumulated << 1;
            accumulated |= u.borrow().val;
            if u.borrow().left.is_none() && u.borrow().right.is_none() {
                *result += accumulated;
                return;
            }
            Solution::_traverse(&u.borrow().left, accumulated, result);
            Solution::_traverse(&u.borrow().right, accumulated, result);
        }
    }

    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        Solution::_traverse(&root, 0, &mut result);
        return result;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn test() {
        assert_eq!(
            Solution::sum_root_to_leaf(TreeNode::from_vec(vec![
                Some(1),
                Some(0),
                Some(1),
                Some(0),
                Some(1),
                Some(0),
                Some(1)
            ])),
            22
        );
    }
}
