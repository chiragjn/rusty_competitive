// https://leetcode.com/problems/delete-node-in-a-bst/

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
    fn min_val(node: Rc<RefCell<TreeNode>>) -> i32 {
        if node.borrow().left.is_some() {
            return Solution::min_val(node.borrow().left.clone().unwrap());
        }
        return node.borrow().val;
    }

    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = root {
            if n.borrow().val == key {
                if n.borrow().left.is_none() && n.borrow().right.is_none() {
                    return None;
                } else if n.borrow().left.is_some() && n.borrow().right.is_none() {
                    return n.borrow_mut().left.take();
                } else if n.borrow().left.is_none() && n.borrow().right.is_some() {
                    return n.borrow_mut().right.take();
                } else {
                    let k = Solution::min_val(n.borrow().right.clone().unwrap());
                    n.borrow_mut().val = k;
                    let right = n.borrow_mut().right.take();
                    n.borrow_mut().right = Solution::delete_node(right, k);
                    return Some(n);
                }
            } else {
                let left = n.borrow_mut().left.take();
                n.borrow_mut().left = Solution::delete_node(left, key);
                let right = n.borrow_mut().right.take();
                n.borrow_mut().right = Solution::delete_node(right, key);
                return Some(n);
            }
        }
        return None;
    }
}

fn main() {
    Solution::delete_node(None, -1);
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn test() {
        assert_eq!(
            Solution::delete_node(
                TreeNode::from_vec(vec![
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    Some(7)
                ]),
                3
            ),
            TreeNode::from_vec(vec![
                Some(5),
                Some(4),
                Some(6),
                Some(2),
                None,
                None,
                Some(7)
            ])
        );

        assert_eq!(
            Solution::delete_node(
                TreeNode::from_vec(vec![
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    Some(7)
                ]),
                5
            ),
            TreeNode::from_vec(vec![Some(6), Some(3), Some(7), Some(2), Some(4)])
        );
    }
}
