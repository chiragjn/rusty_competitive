// https://leetcode.com/problems/range-sum-of-bst/

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
    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;
        if let Some(tn) = node.as_ref() {
            let this = tn.borrow().val;
            if this >= low && this <= high {
                sum += this;
            }
            if this >= low {
                sum += Solution::traverse(&tn.borrow().left, low, high);
            }
            if this <= high {
                sum += Solution::traverse(&tn.borrow().right, low, high);
            }
        }
        return sum;
    }

    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        return Solution::traverse(&root, low, high);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn test() {
        assert_eq!(
            Solution::range_sum_bst(
                TreeNode::from_vec(vec![
                    Some(10),
                    Some(5),
                    Some(15),
                    Some(3),
                    Some(7),
                    None,
                    Some(18)
                ]),
                7,
                15
            ),
            32
        );
        assert_eq!(
            Solution::range_sum_bst(
                TreeNode::from_vec(vec![
                    Some(10),
                    Some(5),
                    Some(15),
                    Some(3),
                    Some(7),
                    Some(13),
                    Some(18),
                    Some(1),
                    None,
                    Some(6)
                ]),
                6,
                10
            ),
            23
        );
    }
}
