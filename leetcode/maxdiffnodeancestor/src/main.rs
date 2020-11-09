// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/

use std::cell::RefCell;
use std::cmp::{max, min};
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
    fn traverse(
        node: &Option<Rc<RefCell<TreeNode>>>,
        answer: &mut i32,
    ) -> (Option<i32>, Option<i32>) {
        if let Some(tn) = node.as_ref() {
            let this = tn.borrow().val;
            let (lmn, lmx) = Solution::traverse(&tn.borrow().left, answer);
            let (rmn, rmx) = Solution::traverse(&tn.borrow().right, answer);
            let lmn = lmn.unwrap_or(this);
            let lmx = lmx.unwrap_or(this);
            let rmn = rmn.unwrap_or(this);
            let rmx = rmx.unwrap_or(this);
            *answer = *[
                *answer,
                (this - lmn).abs(),
                (this - lmx).abs(),
                (this - rmn).abs(),
                (this - rmx).abs(),
            ]
            .iter()
            .max()
            .unwrap();
            return (
                Some(min(this, min(lmn, rmn))),
                Some(max(this, max(lmx, rmx))),
            );
        }
        return (None, None);
    }
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
            Solution::max_ancestor_diff(TreeNode::from_vec(vec![
                Some(8),
                Some(3),
                Some(10),
                Some(1),
                Some(6),
                None,
                Some(14),
                None,
                None,
                Some(4),
                Some(7),
                Some(13)
            ])),
            7
        );
        assert_eq!(
            Solution::max_ancestor_diff(TreeNode::from_vec(vec![
                Some(1),
                None,
                Some(2),
                None,
                Some(0),
                Some(3)
            ])),
            3
        );
    }
}
