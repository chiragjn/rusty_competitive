// https://leetcode.com/problems/sum-of-left-leaves/

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

type ONode = Option<Rc<RefCell<TreeNode>>>;

#[derive(PartialEq, Eq)]
enum NodeType {
    Root,
    Left,
    Right,
}

struct Solution {}

impl Solution {
    fn _dfs(node: &ONode, node_type: NodeType) -> i32 {
        let mut sum = 0;
        if let Some(n) = node {
            if n.borrow().left.is_none()
                && n.borrow().right.is_none()
                && node_type == NodeType::Left
            {
                sum = n.borrow().val;
            } else {
                sum += Solution::_dfs(&n.borrow().left, NodeType::Left);
                sum += Solution::_dfs(&n.borrow().right, NodeType::Right);
            }
        }
        return sum;
    }
    pub fn sum_of_left_leaves(root: ONode) -> i32 {
        return Solution::_dfs(&root, NodeType::Root);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn test() {
        assert_eq!(
            Solution::sum_of_left_leaves(TreeNode::from_vec(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            24
        );
    }
}
