// https://leetcode.com/problems/all-elements-in-two-binary-search-trees/

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

impl TreeNode {
    fn inorder(&self, buffer: &mut Vec<i32>) {
        if let Some(n) = self.left.as_ref() {
            n.borrow().inorder(buffer);
        }
        buffer.push(self.val);
        if let Some(n) = self.right.as_ref() {
            n.borrow().inorder(buffer);
        }
    }
}

struct Solution {}

impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut r1_in = vec![];
        let mut r2_in = vec![];
        let mut answer = vec![];
        if let Some(r1) = root1 {
            r1.borrow().inorder(&mut r1_in);
        }
        if let Some(r2) = root2 {
            r2.borrow().inorder(&mut r2_in);
        }
        let (mut idx1, mut idx2) = (0, 0);
        while idx1 < r1_in.len() && idx2 < r2_in.len() {
            if r1_in[idx1] < r2_in[idx2] {
                answer.push(r1_in[idx1]);
                idx1 += 1;
            } else {
                answer.push(r2_in[idx2]);
                idx2 += 1;
            }
        }
        answer.extend(r1_in[idx1..].iter());
        answer.extend(r2_in[idx2..].iter());
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
            Solution::get_all_elements(
                TreeNode::from_vec(vec![Some(2), Some(1), Some(4)]),
                TreeNode::from_vec(vec![Some(1), Some(0), Some(3)])
            ),
            vec![0, 1, 1, 2, 3, 4]
        );

        assert_eq!(
            Solution::get_all_elements(
                TreeNode::from_vec(vec![Some(0), Some(-10), Some(10)]),
                TreeNode::from_vec(vec![Some(5), Some(1), Some(7), Some(0), Some(2)])
            ),
            vec![-10, 0, 0, 1, 2, 5, 7, 10]
        );

        assert_eq!(
            Solution::get_all_elements(
                TreeNode::from_vec(vec![]),
                TreeNode::from_vec(vec![Some(5), Some(1), Some(7), Some(0), Some(2)])
            ),
            vec![0, 1, 2, 5, 7]
        );

        assert_eq!(
            Solution::get_all_elements(
                TreeNode::from_vec(vec![Some(0), Some(-10), Some(10)]),
                TreeNode::from_vec(vec![])
            ),
            vec![-10, 0, 10]
        );
    }
}
