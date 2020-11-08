// https://leetcode.com/problems/binary-tree-tilt/

use std::cell::RefCell;
use std::cmp::min;
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
    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, answer: &mut i32) -> i32 {
        if let Some(tn) = node.as_ref() {
            let left = Solution::traverse(&tn.borrow().left, answer);
            let right = Solution::traverse(&tn.borrow().right, answer);
            *answer += (left - right).abs();
            return tn.borrow().val + left + right;
        }
        return 0;
    }
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut answer = 0;
        Solution::traverse(&root, &mut answer);
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
            Solution::find_tilt(TreeNode::from_vec(vec![Some(1), Some(2), Some(3)])),
            1
        );
        assert_eq!(
            Solution::find_tilt(TreeNode::from_vec(vec![
                Some(4),
                Some(2),
                Some(9),
                Some(3),
                Some(5),
                None,
                Some(7)
            ])),
            15
        );
        assert_eq!(
            Solution::find_tilt(TreeNode::from_vec(vec![
                Some(21),
                Some(7),
                Some(14),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(3)
            ])),
            9
        );
    }
}
